import logging
import sys
import os
import asyncio
from aiogram import Bot, Dispatcher, executor
from aiogram.contrib.fsm_storage.memory import MemoryStorage

# Import configuration
from config import (
    BOT_TOKEN, 
    DEBUG, 
    LOG_FILE,
    WEBHOOK_URL,
    WEBHOOK_PATH,
    WEBAPP_HOST,
    WEBAPP_PORT
)

# Import handlers
from handlers import register_all_handlers
from handlers.callback_handlers import init_bot as init_callback_handlers

# Configure logging
logging.basicConfig(
    level=logging.DEBUG if DEBUG else logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler(LOG_FILE),
        logging.StreamHandler(sys.stdout)
    ]
)

logger = logging.getLogger(__name__)

# Create data directory if it doesn't exist
os.makedirs("data", exist_ok=True)

# Initialize bot and dispatcher
bot = Bot(token=BOT_TOKEN)
storage = MemoryStorage()
dp = Dispatcher(bot, storage=storage)

async def on_startup(dp):
    """
    Startup actions
    
    Args:
        dp: Dispatcher instance
    """
    # Log startup
    logger.info("Starting bot...")
    
    # Initialize callback handlers with bot instance
    init_callback_handlers(bot)
    
    # Register all handlers
    await register_all_handlers(dp)
    
    # Set webhook if URL is provided
    if WEBHOOK_URL:
        await bot.set_webhook(WEBHOOK_URL + WEBHOOK_PATH)
        logger.info(f"Webhook set to {WEBHOOK_URL + WEBHOOK_PATH}")
    
    logger.info("Bot started successfully!")

async def on_shutdown(dp):
    """
    Shutdown actions
    
    Args:
        dp: Dispatcher instance
    """
    logger.info("Shutting down...")
    
    # Close storage
    await dp.storage.close()
    await dp.storage.wait_closed()
    
    # Close session
    await bot.session.close()
    
    if WEBHOOK_URL:
        await bot.delete_webhook()
    
    logger.info("Bot shut down successfully!")

def main():
    """Main entry point"""
    logger.info("Bot initialization...")
    
    # Start polling or webhook based on configuration
    if WEBHOOK_URL:
        # Start webhook
        executor.start_webhook(
            dispatcher=dp,
            webhook_path=WEBHOOK_PATH,
            on_startup=on_startup,
            on_shutdown=on_shutdown,
            skip_updates=True,
            host=WEBAPP_HOST,
            port=WEBAPP_PORT,
        )
    else:
        # Start polling
        executor.start_polling(
            dispatcher=dp,
            on_startup=on_startup,
            on_shutdown=on_shutdown,
            skip_updates=True
        )

if __name__ == "__main__":
    main()
