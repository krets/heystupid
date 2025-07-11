import configparser
import os
import platform
import socket
import sys
from datetime import datetime
import getpass

import requests
import argparse

try:
    import psutil
except ImportError:
    psutil = None

system_stats = {
    "datetime": datetime.now().isoformat(),
    "os": platform.system(),
    "os_release": platform.release(),
    "python_version": platform.python_version(),
    "hostname": socket.gethostname(),
    "user": getpass.getuser(),
    "cwd": os.getcwd(),
}

if psutil is not None:
    system_stats.update({
        "cpu_count": psutil.cpu_count(logical=True),
        "memory_total_mb": round(psutil.virtual_memory().total / (1024**2)),
        "memory_available_mb": round(psutil.virtual_memory().available / (1024**2)),
        "uptime_sec": int(psutil.boot_time()),
    })
else:
    system_stats.update({
        "cpu_count": os.cpu_count(),
    })

class Defaults:
    model =  'gpt-4.1-mini'
    base_prompt = (
        "This is a command line tool that accepts command output and a user prompt. "
        "Responses should be concise and formatted to wrap at 80 characters long. "
        "Do not include formatting characters or markdown. "
        "Multi-line output is acceptable. "
        "Avoid praise and filler text. "
        "Respond with summations or evaluations of errors to help the user."
    )


def load_config(config_file=None):
    if config_file is None:
        config_file = os.path.join(os.path.expanduser('~'), '.heystupid.config')
    config = configparser.ConfigParser()
    section = 'default'
    with open(config_file, 'r') as stream:
        config.read_string(f'[{section}]\n' + stream.read())
    items = dict(config.items(section))

    settings = Defaults()
    model = items.get('model')
    if model:
        settings.model = model
    base_prompt = items.get('base_prompt')
    if base_prompt:
        settings.base_prompt = base_prompt

    settings.openai_api_key = items.get('openai_api_key')
    return settings


def main():
    parser = argparse.ArgumentParser(description='Ask a question to OpenAI')
    parser.add_argument('prompt', nargs='?', help='The prompt question to ask OpenAI')
    parser.add_argument('--model', default=Defaults.model, help='OpenAI model to use (default: %(default)s)')

    settings = load_config()
    args = parser.parse_args()

    stdin_input = sys.stdin.read().strip() if not sys.stdin.isatty() else ""

    if not any([args.prompt, stdin_input]):
        sys.exit("Error: No input provided.\nUsage examples:\n  heystupid 'What is Rust?'\n  echo 'some text' | heystupid 'Explain this'\n  ls /etc/ | heystupid 'What OS is this?'")

    url = "https://api.openai.com/v1/chat/completions"
    headers = {
        "Authorization": f"Bearer {settings.openai_api_key}",
        "Content-Type": "application/json"
    }
    messages = [
        {"role": "system", "content": str(system_stats)},
        {"role": "system", "content": settings.base_prompt},
    ]
    data = {
        "model": args.model,
        "messages": messages
    }
    if stdin_input:
        messages.append({'role': 'user', 'content': f'stdin: {stdin_input}'})
    if args.prompt:
        messages.append({'role': 'system', 'content': args.prompt})

    try:
        response = requests.post(url, headers=headers, json=data)
        response.raise_for_status()
    except requests.RequestException as e:
        sys.exit(f"Failed to send request to OpenAI: {e}")

    response_data = response.json()
    if response_data.get("choices"):
        print(response_data["choices"][0]["message"]["content"])
    else:
        sys.exit("No response received from OpenAI")

if __name__ == "__main__":
    main()
