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
import httpx
# Create an MCP server
mcp = FastMCP("Demo")


# Add an addition tool
@mcp.tool()
def add(a: int, b: int) -> int:
    """Add two numbers"""
    return a + b


@mcp.tool()
async def fetch_url(url: str) -> str:
    """Fetch url text response"""
    async with httpx.AsyncClient() as client:
        response = await client.get(f"{url}")
        if response.status_code != 200:
            raise Exception(f"Failed to fetch {url}")
        return response.text

# Add a dynamic greeting resource

@mcp.resource("greeting://{name}")
def get_greeting(name: str) -> str:
    """Get a personalized greeting"""
    return f"Hello, {name}!"
