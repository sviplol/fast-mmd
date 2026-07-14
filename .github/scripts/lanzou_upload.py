#!/usr/bin/env python3
"""Lanzou Cloud CLI — pure Python stdlib, no pip install needed.

Usage:
  Set env vars: YLOGIN, PHPDISK_INFO, FASTMODELS_FID

Commands:
  upload <file_or_dir>          Upload file(s) to target folder (delete old, upload, move)
  mkdir <name> [parent_id]      Create folder (parent_id defaults to 0=root)
  list [folder_id]              List folders (no arg) or files in folder
  share <file_id>               Get file share link
  share-folder <folder_id>      Get folder share link
  delete <file_id>              Delete file
  move <file_id> <folder_id>    Move file to folder
  rename-folder <new_name> <folder_id>  Rename folder

Requires: Python 3 only (urllib, json, ssl, uuid, os, sys, time, glob, urllib.parse)
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
_ssl_ctx.check_hostname = False
_ssl_ctx.verify_mode = ssl.CERT_NONE


def _get_creds():
    """Get credentials from env vars, with built-in defaults."""
    ylogin = os.environ.get("YLOGIN", "344454")
    phpdisk_info = os.environ.get(
        "PHPDISK_INFO",
        "AjUCNFI0ADgPOFA0DV5QO1Y%2BBQ5dd1NtVCQCOlM8UDlVNABoBD8EAQ8%2BAmYAO1FvUjQGZgllATUPOFMzAGUGNAJlAmJSMAA%2FDzxQMw0zUG5WNQU0XWJTM1RkAmNTYlBkVTcAYwQ3BDEPBAJjAGlRO1I1BjEJYgFvDz1TOgAyBj0CDwIxUjcANA8%2BUDYNZVA%2FVjEFNV01",
    )
    folder_id = os.environ.get("FASTMODELS_FID", "2008280")
    return ylogin, phpdisk_info, folder_id


def _make_cookie(ylogin, phpdisk_info):
    return f"ylogin={ylogin}; phpdisk_info={phpdisk_info}"


def build_multipart(fields, files):
    """Build a multipart/form-data body using pure stdlib."""
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
    try:
        with urllib.request.urlopen(req, timeout=30, context=_ssl_ctx) as resp:
            return json.loads(resp.read().decode("utf-8", errors="replace"))
    except urllib.error.HTTPError as e:
        return {"zt": -1, "info": f"HTTP {e.code}", "error": True}
    except Exception as e:
        return {"zt": -1, "info": str(e), "error": True}


# ============================================================
# API Functions
# ============================================================

def list_folder_files(folder_id, ylogin, phpdisk_info):
    """List files in a Lanzou folder."""
    result = api_call(
        {"task": "5", "folder_id": folder_id, "pg": "1"},
        ylogin, phpdisk_info,
    )
    files = result.get("text", [])
    return files if isinstance(files, list) else []


def list_folders(ylogin, phpdisk_info):
    """List all folders in root."""
    result = api_call({"task": "47", "folder_id": "-1"}, ylogin, phpdisk_info)
    folders = result.get("text", [])
    return folders if isinstance(folders, list) else []


def create_folder(name, parent_id, ylogin, phpdisk_info):
    """Create a new folder. parent_id=0 for root."""
    result = api_call(
        {"task": "4", "folder_name": name, "folder_id": parent_id},
        ylogin, phpdisk_info,
    )
    return result


def delete_file(file_id, ylogin, phpdisk_info):
    """Delete a file by its numeric id."""
    result = api_call({"task": "6", "file_id": file_id}, ylogin, phpdisk_info)
    return result


def delete_folder(folder_id, ylogin, phpdisk_info):
    """Delete a folder by its fol_id."""
    result = api_call({"task": "3", "folder_id": folder_id}, ylogin, phpdisk_info)
    return result


def move_file(file_id, folder_id, ylogin, phpdisk_info):
    """Move a file to a folder. file_id must be numeric id, NOT f_id."""
    result = api_call(
        {"task": "20", "file_id": file_id, "folder_id": folder_id},
        ylogin, phpdisk_info,
    )
    return result


def get_file_share(file_id, ylogin, phpdisk_info):
    """Get share link for a file."""
    result = api_call({"task": "22", "file_id": file_id}, ylogin, phpdisk_info)
    return result


def get_folder_share(folder_id, ylogin, phpdisk_info):
    """Get share link for a folder."""
    result = api_call({"task": "18", "folder_id": folder_id}, ylogin, phpdisk_info)
    return result


def upload_file(filepath, ylogin, phpdisk_info):
    """Upload a single file to Lanzou root, return numeric file_id or None.

    Lanzou's html5up.php returns both 'id' (numeric, used for move/delete)
    and 'f_id' (string hash, used for share links). We need the numeric id.
    """
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
        "lastModifiedDate": time.strftime("%a %b %d %Y %H:%M:%S GMT+0800"),
        "size": str(filesize),
    }

    with open(filepath, "rb") as f:
        files = {"upload_file": (filename, f, "application/octet-stream")}
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
            if text_list:
                file_id = text_list[0].get("id", "")
                f_id = text_list[0].get("f_id", "")
                print(f"  Upload OK, id={file_id} f_id={f_id}")
                return file_id
            return None
        else:
            print(f"  Upload failed: {result.get('info', 'unknown')}")
            return None
    except (json.JSONDecodeError, IndexError) as e:
        print(f"  Parse error: {e}")
        return None


def process_upload(filepath, ylogin, phpdisk_info, folder_id):
    """Full upload workflow: delete old -> upload (with retry) -> move."""
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

    # Step 2: Upload to root with retry (html5up.php ignores folder_id)
    file_id = None
    for attempt in range(3):
        if attempt > 0:
            wait = 10 * attempt
            print(f"  Retry {attempt+1}/3 after {wait}s...")
            time.sleep(wait)
        file_id = upload_file(filepath, ylogin, phpdisk_info)
        if file_id:
            break
        print(f"  Upload attempt {attempt+1} failed")
    if not file_id:
        print(f"  All upload attempts failed for {filename}")
        return False

    # Step 3: Move from root to target folder
    print(f"  Moving to folder {folder_id}...")
    try:
        result = move_file(file_id, folder_id, ylogin, phpdisk_info)
        if result.get("zt") == 1:
            print(f"  Moved OK")
            return True
        else:
            print(f"  Move failed: {result.get('info')}, file stays in root")
            return False
    except Exception as e:
        print(f"  Move error: {e}")
        return False


# ============================================================
# CLI Commands
# ============================================================

def cmd_upload(args):
    """Upload file(s) to target folder."""
    if not args:
        print("Usage: upload <file_or_dir>")
        sys.exit(1)
    target = args[0]
    ylogin, phpdisk_info, folder_id = _get_creds()

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
        ok = process_upload(filepath, ylogin, phpdisk_info, folder_id)
        if not ok:
            all_ok = False

    # Print share links for uploaded files
    if all_ok:
        print("\n=== Share Links ===")
        folder_files = list_folder_files(folder_id, ylogin, phpdisk_info)
        for f in folder_files:
            name = f.get("name_all", "")
            fid = f.get("id", "")
            if name in [os.path.basename(fp) for fp in files]:
                share = get_file_share(fid, ylogin, phpdisk_info)
                info = share.get("info", {})
                if isinstance(info, dict):
                    f_id = info.get("f_id", "")
                    newd = info.get("is_newd", "")
                    pwd = info.get("pwd", "")
                    link = f"{newd}/{f_id}" if f_id and newd else "N/A"
                    print(f"  {name}: {link} (pwd: {pwd})")

    sys.exit(0 if all_ok else 1)


def cmd_mkdir(args):
    """Create a new folder."""
    if not args:
        print("Usage: mkdir <name> [parent_id]")
        sys.exit(1)
    name = args[0]
    parent_id = args[1] if len(args) > 1 else "0"
    ylogin, phpdisk_info, _ = _get_creds()

    result = create_folder(name, parent_id, ylogin, phpdisk_info)
    if result.get("zt") == 1:
        print(f"Folder '{name}' created in parent {parent_id}")
        # Find the new folder's fol_id
        folders = list_folders(ylogin, phpdisk_info)
        for f in folders:
            if f.get("name") == name:
                fol_id = f.get("fol_id", "")
                print(f"  fol_id: {fol_id}")
                # Get share link
                share = get_folder_share(fol_id, ylogin, phpdisk_info)
                info = share.get("info", {})
                if isinstance(info, dict):
                    new_url = info.get("new_url", "")
                    pwd = info.get("pwd", "")
                    print(f"  Share: {new_url} (pwd: {pwd})")
                break
    else:
        print(f"Create failed: {result.get('info')}")


def cmd_list(args):
    """List folders or files in a folder."""
    ylogin, phpdisk_info, _ = _get_creds()

    if args:
        folder_id = args[0]
        print(f"=== Files in folder {folder_id} ===")
        files = list_folder_files(folder_id, ylogin, phpdisk_info)
        if not files:
            print("  (empty)")
        for f in files:
            name = f.get("name_all", f.get("name", ""))
            fid = f.get("id", "")
            size = f.get("size", "")
            t = f.get("time", "")
            print(f"  {name:45} id={fid:15} size={size:10} time={t}")
    else:
        print("=== Folders ===")
        folders = list_folders(ylogin, phpdisk_info)
        for f in folders:
            name = f.get("name", "")
            fol_id = f.get("fol_id", "")
            print(f"  {name:45} fol_id={fol_id}")


def cmd_share(args):
    """Get file share link."""
    if not args:
        print("Usage: share <file_id>")
        sys.exit(1)
    file_id = args[0]
    ylogin, phpdisk_info, _ = _get_creds()

    result = get_file_share(file_id, ylogin, phpdisk_info)
    info = result.get("info", {})
    if isinstance(info, dict):
        f_id = info.get("f_id", "")
        newd = info.get("is_newd", "")
        pwd = info.get("pwd", "")
        onof = info.get("onof", "0")
        link = f"{newd}/{f_id}" if f_id and newd else "N/A"
        print(f"Share link: {link}")
        print(f"Password: {pwd if onof == '1' else '(no password)'}")
    else:
        print(f"Failed: {result}")


def cmd_share_folder(args):
    """Get folder share link."""
    if not args:
        print("Usage: share-folder <folder_id>")
        sys.exit(1)
    folder_id = args[0]
    ylogin, phpdisk_info, _ = _get_creds()

    result = get_folder_share(folder_id, ylogin, phpdisk_info)
    info = result.get("info", {})
    if isinstance(info, dict):
        name = info.get("name", "")
        new_url = info.get("new_url", "")
        pwd = info.get("pwd", "")
        onof = info.get("onof", "0")
        print(f"Folder: {name}")
        print(f"Share link: {new_url}")
        print(f"Password: {pwd if onof == '1' else '(no password)'}")
    else:
        print(f"Failed: {result}")


def cmd_delete(args):
    """Delete a file."""
    if not args:
        print("Usage: delete <file_id>")
        sys.exit(1)
    file_id = args[0]
    ylogin, phpdisk_info, _ = _get_creds()

    result = delete_file(file_id, ylogin, phpdisk_info)
    if result.get("zt") == 1:
        print(f"Deleted file {file_id}")
    else:
        print(f"Delete failed: {result.get('info')}")


def cmd_move(args):
    """Move a file to a folder."""
    if len(args) < 2:
        print("Usage: move <file_id> <folder_id>")
        sys.exit(1)
    file_id = args[0]
    folder_id = args[1]
    ylogin, phpdisk_info, _ = _get_creds()

    result = move_file(file_id, folder_id, ylogin, phpdisk_info)
    if result.get("zt") == 1:
        print(f"Moved file {file_id} to folder {folder_id}")
    else:
        print(f"Move failed: {result.get('info')}")


def cmd_rename_folder(args):
    """Rename a folder."""
    if len(args) < 2:
        print("Usage: rename-folder <new_name> <folder_id>")
        sys.exit(1)
    new_name = args[0]
    folder_id = args[1]
    ylogin, phpdisk_info, _ = _get_creds()

    result = create_folder(new_name, folder_id, ylogin, phpdisk_info)
    if result.get("zt") == 1:
        print(f"Renamed folder {folder_id} to '{new_name}'")
    else:
        print(f"Rename failed: {result.get('info')}")


# ============================================================
# Main
# ============================================================

COMMANDS = {
    "upload": cmd_upload,
    "mkdir": cmd_mkdir,
    "list": cmd_list,
    "share": cmd_share,
    "share-folder": cmd_share_folder,
    "delete": cmd_delete,
    "move": cmd_move,
    "rename-folder": cmd_rename_folder,
}

USAGE = """Lanzou Cloud CLI (pure stdlib, no pip needed)

Usage: lanzou_upload.py <command> [args]

Commands:
  upload <file_or_dir>              Upload file(s) to target folder
  mkdir <name> [parent_id]          Create folder (parent_id=0 for root)
  list [folder_id]                  List folders (no arg) or files in folder
  share <file_id>                   Get file share link
  share-folder <folder_id>          Get folder share link
  delete <file_id>                  Delete file
  move <file_id> <folder_id>        Move file to folder
  rename-folder <new_name> <folder_id>  Rename folder

Env vars (with defaults):
  YLOGIN=344454
  PHPDISK_INFO=AjUCNFI0ADgPOFA0DV5QO1Y...
  FASTMODELS_FID=2008280
"""


def main():
    if len(sys.argv) < 2 or sys.argv[1] in ("-h", "--help", "help"):
        print(USAGE)
        sys.exit(0)

    cmd = sys.argv[1]
    args = sys.argv[2:]

    if cmd not in COMMANDS:
        print(f"Unknown command: {cmd}")
        print(USAGE)
        sys.exit(1)

    COMMANDS[cmd](args)


if __name__ == "__main__":
    main()
