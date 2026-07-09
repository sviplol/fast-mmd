#!/usr/bin/env python3
"""Upload files to Lanzou Cloud (Fastmodels folder) using pure stdlib.

Usage: python3 lanzou_upload.py <dir_or_file>

Uses urllib + manual multipart construction — no pip install needed.
"""
import os
import sys
import time
import glob
import json
import uuid
import urllib.request
import urllib.error
import ssl

UPLOAD_URL = "https://pc.woozooo.com/html5up.php"
REFERER = "https://pc.woozooo.com/mydisk.php"
USER_AGENT = (
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
    "AppleWebKit/537.36 (KHTML, like Gecko) "
    "Chrome/120.0.0.0 Safari/537.36"
)


def build_multipart(fields, files):
    """Build a multipart/form-data body using pure stdlib.

    fields: dict of str -> str
    files:  dict of fieldname -> (filename, fileobj, content_type)
    Returns (body_bytes, content_type_header)
    """
    boundary = "----WebKitFormBoundary" + uuid.uuid4().hex[:16]
    lines = []

    # Text fields
    for key, value in fields.items():
        lines.append(f"--{boundary}".encode())
        lines.append(
            f'Content-Disposition: form-data; name="{key}"'.encode()
        )
        lines.append(b"")
        lines.append(str(value).encode())

    # File fields
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

    # Closing boundary
    lines.append(f"--{boundary}--".encode())
    lines.append(b"")

    body = b"\r\n".join(lines)
    content_type = f"multipart/form-data; boundary={boundary}"
    return body, content_type


def upload_file(filepath, ylogin, phpdisk_info, folder_id):
    """Upload a single file to Lanzou Cloud."""
    filename = os.path.basename(filepath)
    filesize = os.path.getsize(filepath)
    print(f"Uploading {filename} ({filesize} bytes) to folder {folder_id}")

    fields = {
        "task": "1",
        "vie": "2",
        "ve": "2",
        "folder_id": folder_id,
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

    cookie = f"ylogin={ylogin}; phpdisk_info={phpdisk_info}"

    req = urllib.request.Request(UPLOAD_URL, data=body, method="POST")
    req.add_header("User-Agent", USER_AGENT)
    req.add_header("Cookie", cookie)
    req.add_header("Referer", REFERER)
    req.add_header("Content-Type", content_type)

    ctx = ssl.create_default_context()

    try:
        with urllib.request.urlopen(req, timeout=600, context=ctx) as resp:
            status = resp.status
            resp_body = resp.read().decode("utf-8", errors="replace")
    except urllib.error.HTTPError as e:
        status = e.code
        resp_body = e.read().decode("utf-8", errors="replace")
    except Exception as e:
        print(f"Upload error: {e}")
        return False

    print(f"HTTP Status: {status}")
    print(f"Response: {resp_body[:500]}")

    try:
        result = json.loads(resp_body)
        if result.get("zt") == 1:
            text_list = result.get("text", [{}])
            file_id = text_list[0].get("f_id", "") if text_list else ""
            print(f"Upload success! File ID: {file_id}")
            return True
        else:
            print(f"Upload failed: {result.get('info', 'unknown')}")
            return False
    except (json.JSONDecodeError, IndexError) as e:
        print(f"Parse error: {e}")
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

    # Gather files
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
        ok = upload_file(filepath, ylogin, phpdisk_info, folder_id)
        if not ok:
            all_ok = False

    sys.exit(0 if all_ok else 1)


if __name__ == "__main__":
    main()
