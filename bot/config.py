import os
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

# Bot configuration
BOT_TOKEN = os.getenv("BOT_TOKEN")
if not BOT_TOKEN:
    raise ValueError("No BOT_TOKEN found in environment variables!")

# Admin configuration
ADMIN_ID = os.getenv("ADMIN_ID")

# Data file paths
DATA_FILE = os.getenv("DATA_FILE", "data/bot_data.json")
LOG_FILE = os.getenv("LOG_FILE", "data/user_interactions.log")

# Other settings
DEBUG = os.getenv("DEBUG", "False").lower() in ("true", "1", "t")
WEBHOOK_URL = os.getenv("WEBHOOK_URL", None)
WEBHOOK_PATH = os.getenv("WEBHOOK_PATH", "/webhook")
WEBAPP_HOST = os.getenv("WEBAPP_HOST", "0.0.0.0")
WEBAPP_PORT = int(os.getenv("WEBAPP_PORT", 8000))

# Bot configuration
BOT_NAME = "Верхневолжский ГАУ Бот"
BOT_DESCRIPTION = "Информационный бот приемной комиссии Верхневолжского ГАУ"

# Pagination settings
ITEMS_PER_PAGE = 5 