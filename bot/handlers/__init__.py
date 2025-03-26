"""Handlers package initialization"""

from handlers.command_handlers import register_command_handlers
from handlers.callback_handlers import register_callback_handlers
from handlers.message_handlers import register_message_handlers
from handlers.search_handlers import register_search_handlers

async def register_all_handlers(dp):
    """
    Register all handlers in the correct order
    
    Args:
        dp: Dispatcher instance
    """
    # Register handlers in order of priority
    await register_command_handlers(dp)
    await register_callback_handlers(dp)
    await register_search_handlers(dp)
    await register_message_handlers(dp)  # Message handlers should be last 