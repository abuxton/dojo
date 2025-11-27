# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "httpx",
#     "mcp",
#     "os",
# ]
# ///

# server.py

from mcp.server.fastmcp import FastMCP
import os
# Create an MCP server
mcp = FastMCP("OAI")

OBSIDIAN_FILE = "oai.md"
OBSIDIAN_PATH = "/Users/abuxton/Dropbox/documents/obsidian/MCP-OAI"

def ensure_file():
    """_summary_
	Ensure the obsidian file exists
	"""
    if not os.path.exists(OBSIDIAN_PATH+"/"+OBSIDIAN_FILE):
        with open(OBSIDIAN_FILE, "w") as f:
            f.write("# OAI\n")

# Add an addition tool


@mcp.tool()
def add_note(message: str) -> str:
	"""Add a note to the obsidian file"
	Args:
		message (str): The message to add to the obsidian file
	returns:
		str: The message added to the obsidian file
	"""

	ensure_file()
	with open(OBSIDIAN_PATH+"/"+OBSIDIAN_FILE, "a") as f:
		f.write(f"- {message}\n")
	return f"Added note: {message}"
