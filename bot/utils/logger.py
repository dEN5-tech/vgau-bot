import logging
import json
from datetime import datetime
from config import LOG_FILE

# Configure logging
logging.basicConfig(
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    level=logging.INFO,
    filename=LOG_FILE
)
logger = logging.getLogger(__name__)

def log_user_interaction(user_id, action, additional_data=None):
    """
    Log user interactions with the bot
    
    Args:
        user_id (int): Telegram user ID
        action (str): Type of action performed
        additional_data (dict, optional): Additional data to log
    """
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    log_entry = {
        "timestamp": timestamp,
        "user_id": user_id,
        "action": action,
        "additional_data": additional_data
    }
    
    logger.info(f"User interaction: {log_entry}")
    
    # You can also save to a separate JSON file if needed
    # try:
    #     with open('data/user_logs.json', 'a', encoding='utf-8') as file:
    #         file.write(json.dumps(log_entry) + '\n')
    # except Exception as e:
    #     logger.error(f"Error logging user interaction: {e}")
    
def get_logger():
    """Get the configured logger"""
    return logger 