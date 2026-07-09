#!/usr/bin/env python3
"""Upload files to Lanzou Cloud (Fastmodels folder) using pure stdlib.

Usage: python3 lanzou_upload.py <dir_or_file>

Uses urllib + manual multipart construction — no pip install needed.

Workflow:
  1. List existing files in target folder, delete same-name old ones
  2. Upload file (goes to root — html5up.php folder_id param is ignored)
  3. Move uploaded file from root to target folder via doupload.php task=20
"""
import os
import sys
import time
import glob
import json
import uuid
import urllib.request
import urllib.error
import urllib.parse
import ssl

UPLOAD_URL = "https://pc.woozooo.com/html5up.php"
API_URL = "https://pc.woozooo.com/doupload.php"
REFERER = "https://pc.woozooo.com/mydisk.php"
USER_AGENT = (
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
    "AppleWebKit/537.36 (KHTML, like Gecko) "
    "Chrome/120.0.0.0 Safari/537.36"
)

_ssl_ctx = ssl.create_default_context()


def _make_cookie(ylogin, phpdisk_info):
    return f"ylogin={ylogin}; phpdisk_info={phpdisk_info}"


def build_multipart(fields, files):
    """Build a multipart/form-data body using pure stdlib.

    fields: dict of str -> str
    files:  dict of fieldname -> (filename, fileobj, content_type)
    Returns (body_bytes, content_type_header)
    """
    boundary = "----WebKitFormBoundary" + uuid.uuid4().hex[:16]
    lines = []

    for key, value in fields.items():
        lines.append(f"--{boundary}".encode())
        lines.append(f'Content-Disposition: form-data; name="{key}"'.encode())
        lines.append(b"")
        lines.append(str(value).encode())

    for fieldname, (filename, fileobj, content_type) in files.items():
        lines.append(f"--{boundary}".encode())
        lines.append(
            f'Content-Disposition: form-data; name="{fieldname}"; '
            f'filename="{filename}"'.encode()
        )
        lines.append(f"Content-Type: {content_type}".encode())
        lines.append(b"")
        fileobj.seek(0)
        lines.append(fileobj.read())

    lines.append(f"--{boundary}--".encode())
    lines.append(b"")

    body = b"\r\n".join(lines)
    content_type = f"multipart/form-data; boundary={boundary}"
    return body, content_type


def api_call(params, ylogin, phpdisk_info):
    """Call doupload.php with given params, return parsed JSON."""
    data = urllib.parse.urlencode(params).encode()
    req = urllib.request.Request(API_URL, data=data, method="POST")
    req.add_header("User-Agent", USER_AGENT)
    req.add_header("Cookie", _make_cookie(ylogin, phpdisk_info))
    req.add_header("Referer", REFERER)
    req.add_header("Content-Type", "application/x-www-form-urlencoded")
    with urllib.request.urlopen(req, timeout=30, context=_ssl_ctx) as resp:
        return json.loads(resp.read().decode("utf-8", errors="replace"))


def list_folder_files(folder_id, ylogin, phpdisk_info):
    """List files in a Lanzou folder. Returns list of {name_all, id, ...}."""
    result = api_call(
        {"task": "5", "folder_id": folder_id, "pg": "1"},
        ylogin, phpdisk_info,
    )
    files = result.get("text", [])
    return files if isinstance(files, list) else []


def list_root_files(ylogin, phpdisk_info):
    """List files in root directory."""
    result = api_call({"task": "5", "pg": "1"}, ylogin, phpdisk_info)
    files = result.get("text", [])
    return files if isinstance(files, list) else []


def delete_file(file_id, ylogin, phpdisk_info):
    """Delete a file by its id."""
    result = api_call({"task": "6", "file_id": file_id}, ylogin, phpdisk_info)
    return result.get("zt") == 1


def move_file_to_folder(file_id, folder_id, ylogin, phpdisk_info):
    """Move a file from root to a folder. Returns True on success."""
    result = api_call(
        {"task": "20", "file_id": file_id, "folder_id": folder_id},
        ylogin, phpdisk_info,
    )
    return result.get("zt") == 1


def upload_file(filepath, ylogin, phpdisk_info):
    """Upload a single file to Lanzou Cloud root, return file_id or None."""
    filename = os.path.basename(filepath)
    filesize = os.path.getsize(filepath)
    print(f"  Uploading {filename} ({filesize} bytes)")

    fields = {
        "task": "1",
        "vie": "2",
        "ve": "2",
        "id": "WU_FILE_0",
        "name": filename,
        "type": "application/octet-stream",
        "lastModifiedDate": time.strftime(
            "%a %b %d %Y %H:%M:%S GMT+0800"
        ),
        "size": str(filesize),
    }

    with open(filepath, "rb") as f:
        files = {
            "upload_file": (
                filename,
                f,
                "application/octet-stream",
            )
        }
        body, content_type = build_multipart(fields, files)

    cookie = _make_cookie(ylogin, phpdisk_info)

    req = urllib.request.Request(UPLOAD_URL, data=body, method="POST")
    req.add_header("User-Agent", USER_AGENT)
    req.add_header("Cookie", cookie)
    req.add_header("Referer", REFERER)
    req.add_header("Content-Type", content_type)

    try:
        with urllib.request.urlopen(req, timeout=600, context=_ssl_ctx) as resp:
            status = resp.status
            resp_body = resp.read().decode("utf-8", errors="replace")
    except urllib.error.HTTPError as e:
        status = e.code
        resp_body = e.read().decode("utf-8", errors="replace")
    except Exception as e:
        print(f"  Upload error: {e}")
        return None

    print(f"  HTTP {status}: {resp_body[:200]}")

    try:
        result = json.loads(resp_body)
        if result.get("zt") == 1:
            text_list = result.get("text", [{}])
            file_id = text_list[0].get("f_id", "") if text_list else ""
            print(f"  Upload OK, file_id={file_id}")
            return file_id
        else:
            print(f"  Upload failed: {result.get('info', 'unknown')}")
            return None
    except (json.JSONDecodeError, IndexError) as e:
        print(f"  Parse error: {e}")
        return None


def process_file(filepath, ylogin, phpdisk_info, folder_id):
    """Full upload workflow for one file: delete old -> upload -> move."""
    filename = os.path.basename(filepath)
    print(f"\n--- {filename} ---")

    # Step 1: Delete same-name files in target folder
    print("  Checking for old files in target folder...")
    try:
        folder_files = list_folder_files(folder_id, ylogin, phpdisk_info)
        old = [f for f in folder_files if f.get("name_all") == filename]
        for f in old:
            fid = f.get("id", "")
            print(f"  Deleting old: {filename} (id={fid})")
            delete_file(fid, ylogin, phpdisk_info)
    except Exception as e:
        print(f"  Warning: could not list/delete old files: {e}")

    # Step 2: Upload to root (html5up.php ignores folder_id)
    file_id = upload_file(filepath, ylogin, phpdisk_info)
    if not file_id:
        return False

    # Step 3: Move from root to target folder
    print(f"  Moving to folder {folder_id}...")
    try:
        ok = move_file_to_folder(file_id, folder_id, ylogin, phpdisk_info)
        if ok:
            print(f"  Moved OK")
            return True
        else:
            print(f"  Move failed, file stays in root")
            return False
    except Exception as e:
        print(f"  Move error: {e}")
        return False


def main():
    ylogin = os.environ.get("YLOGIN", "")
    phpdisk_info = os.environ.get("PHPDISK_INFO", "")
    folder_id = os.environ.get("FASTMODELS_FID", "")

    if not ylogin or not phpdisk_info or not folder_id:
        print("Missing secrets (YLOGIN/PHPDISK_INFO/FASTMODELS_FID), skip upload")
        sys.exit(0)

    if len(sys.argv) < 2:
        print("Usage: lanzou_upload.py <dir_or_file>")
        sys.exit(1)

    target = sys.argv[1]

    if os.path.isdir(target):
        files = sorted(glob.glob(os.path.join(target, "*")))
    else:
        files = [target]

    files = [f for f in files if os.path.isfile(f)]
    if not files:
        print("No files to upload")
        sys.exit(1)

    all_ok = True
    for filepath in files:
        ok = process_file(filepath, ylogin, phpdisk_info, folder_id)
        if not ok:
            all_ok = False

    sys.exit(0 if all_ok else 1)


if __name__ == "__main__":
    main()
