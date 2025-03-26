import json
import os
from config import DATA_FILE
from utils.logger import get_logger

logger = get_logger()

def load_bot_data():
    """
    Load bot data from JSON file
    
    Returns:
        dict: The loaded bot data
    """
    try:
        with open(DATA_FILE, 'r', encoding='utf-8') as file:
            return json.load(file)
    except Exception as e:
        logger.error(f"Error loading bot data: {e}")
        return {
            "title": "Данные для кнопок для телеграмм бота Верхневолжского ГАУ",
            "main_menu": []
        }

def save_bot_data(data):
    """
    Save bot data to JSON file
    
    Args:
        data (dict): The bot data to save
    """
    try:
        # Ensure data directory exists
        os.makedirs(os.path.dirname(DATA_FILE), exist_ok=True)
        
        with open(DATA_FILE, 'w', encoding='utf-8') as file:
            json.dump(data, file, ensure_ascii=False, indent=2)
            
        return True
    except Exception as e:
        logger.error(f"Error saving bot data: {e}")
        return False

def get_menu_item_by_callback(callback_data):
    """
    Get menu item by its callback data
    
    Args:
        callback_data (str): The callback data to search for
        
    Returns:
        dict: The menu item with the given callback data, or None if not found
    """
    bot_data = load_bot_data()
    
    # Search in main menu
    for item in bot_data.get("main_menu", []):
        if item.get("callback_data") == callback_data:
            return item
        
        # Search in submenus
        for submenu_item in item.get("submenu", []):
            if submenu_item.get("callback_data") == callback_data:
                return submenu_item
            
            # Search in sub-submenus
            for sub_item in submenu_item.get("submenu", []):
                if sub_item.get("callback_data") == callback_data:
                    return sub_item
                    
            # Search in documents
            if "documents" in submenu_item:
                for doc in submenu_item.get("documents", []):
                    if doc.get("callback_data") == callback_data:
                        return doc
    
    # Search in FAQ
    for idx, faq_item in enumerate(bot_data.get("faq", [])):
        if faq_item.get("callback_data") == callback_data:
            return faq_item
            
    return None 