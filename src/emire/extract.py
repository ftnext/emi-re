import re


def extract_json(markdown: str) -> str:
    r"""
    >>> md = '''
    ... Here is some text.
    ... ```json
    ... {
    ...   "key": "value",
    ...   "number": 123
    ... }
    ... ```
    ... More text here.
    ... '''
    >>> extract_json(md)
    '{\n  "key": "value",\n  "number": 123\n}'
    >>> import json
    >>> json.loads(extract_json(md))
    {'key': 'value', 'number': 123}
    """
    pattern = r"```json\s*(.*?)\s*```"
    match = re.search(pattern, markdown, re.DOTALL)
    if match:
        return match.group(1)

    message = "No JSON block found in the provided markdown."
    raise ValueError(message)
