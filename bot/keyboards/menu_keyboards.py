from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton
from utils.data_loader import load_bot_data

def create_main_menu_keyboard():
    """
    Create main menu keyboard
    
    Returns:
        InlineKeyboardMarkup: The main menu keyboard
    """
    bot_data = load_bot_data()
    keyboard = InlineKeyboardMarkup(row_width=1)
    
    for item in bot_data.get("main_menu", []):
        button = InlineKeyboardButton(
            text=item.get("text", ""),
            callback_data=item.get("callback_data", "")
        )
        keyboard.add(button)
    
    return keyboard

def create_submenu_keyboard(callback_data):
    """
    Create submenu keyboard for a given menu item
    
    Args:
        callback_data (str): The callback data of the parent menu item
        
    Returns:
        InlineKeyboardMarkup: The submenu keyboard
    """
    bot_data = load_bot_data()
    keyboard = InlineKeyboardMarkup(row_width=1)
    
    # Find the menu item with the given callback_data
    for item in bot_data.get("main_menu", []):
        if item.get("callback_data") == callback_data and "submenu" in item:
            for submenu_item in item.get("submenu", []):
                if "url" in submenu_item and not "submenu" in submenu_item and not "documents" in submenu_item:
                    # Direct URL link
                    button = InlineKeyboardButton(
                        text=submenu_item.get("text", ""),
                        url=submenu_item.get("url", "")
                    )
                else:
                    # Regular callback button
                    button = InlineKeyboardButton(
                        text=submenu_item.get("text", ""),
                        callback_data=submenu_item.get("callback_data", "")
                    )
                keyboard.add(button)
    
    # Add back button
    keyboard.add(InlineKeyboardButton(text="⬅️ Назад", callback_data="back_to_main"))
    
    return keyboard

def create_sub_submenu_keyboard(parent_callback_data, callback_data):
    """
    Create sub-submenu keyboard for a given submenu item
    
    Args:
        parent_callback_data (str): The callback data of the parent menu item
        callback_data (str): The callback data of the submenu item
        
    Returns:
        InlineKeyboardMarkup: The sub-submenu keyboard
    """
    bot_data = load_bot_data()
    keyboard = InlineKeyboardMarkup(row_width=1)
    
    # Find the submenu item with the given callback_data
    for item in bot_data.get("main_menu", []):
        if item.get("callback_data") == parent_callback_data:
            for submenu_item in item.get("submenu", []):
                if submenu_item.get("callback_data") == callback_data and "submenu" in submenu_item:
                    for sub_item in submenu_item.get("submenu", []):
                        if "url" in sub_item:
                            button = InlineKeyboardButton(
                                text=sub_item.get("text", ""),
                                url=sub_item.get("url", "")
                            )
                        else:
                            button = InlineKeyboardButton(
                                text=sub_item.get("text", ""),
                                callback_data=sub_item.get("callback_data", "")
                            )
                        keyboard.add(button)
    
    # Add back button
    keyboard.add(InlineKeyboardButton(text="⬅️ Назад", callback_data=f"back_to_{parent_callback_data}"))
    
    return keyboard

def create_document_keyboard(documents, parent_callback, page_num=1, parent_menu_callback=None):
    """
    Create keyboard for document list with pagination
    
    Args:
        documents (list): List of document items
        parent_callback (str): The callback data of the parent item
        page_num (int): Current page number
        parent_menu_callback (str, optional): The callback data of the parent menu
        
    Returns:
        InlineKeyboardMarkup: The document keyboard with pagination
    """
    from bot.config import ITEMS_PER_PAGE
    
    keyboard = InlineKeyboardMarkup(row_width=1)
    
    items_per_page = ITEMS_PER_PAGE
    total_pages = (len(documents) + items_per_page - 1) // items_per_page
    
    # Ensure page is within bounds
    page_num = max(1, min(page_num, total_pages))
    
    # Calculate start and end indices
    start_idx = (page_num - 1) * items_per_page
    end_idx = min(start_idx + items_per_page, len(documents))
    
    # Add document buttons
    for doc in documents[start_idx:end_idx]:
        button_text = f"📄 {doc.get('text', 'Документ')}"
        
        if "url" in doc:
            keyboard.add(InlineKeyboardButton(
                text=button_text,
                url=doc.get("url")
            ))
        elif "callback_data" in doc:
            keyboard.add(InlineKeyboardButton(
                text=button_text,
                callback_data=doc.get("callback_data")
            ))
    
    # Add pagination controls if needed
    if total_pages > 1:
        pagination_row = []
        
        if page_num > 1:
            pagination_row.append(InlineKeyboardButton(
                text="◀️",
                callback_data=f"doc_page_{parent_callback}_{page_num-1}"
            ))
        
        pagination_row.append(InlineKeyboardButton(
            text=f"{page_num}/{total_pages}",
            callback_data="pagination_info"
        ))
        
        if page_num < total_pages:
            pagination_row.append(InlineKeyboardButton(
                text="▶️",
                callback_data=f"doc_page_{parent_callback}_{page_num+1}"
            ))
        
        keyboard.row(*pagination_row)
    
    # Add back button
    if parent_menu_callback:
        keyboard.add(InlineKeyboardButton(
            text="⬅️ Назад",
            callback_data=f"back_to_{parent_menu_callback}"
        ))
    else:
        keyboard.add(InlineKeyboardButton(
            text="⬅️ Назад",
            callback_data="back_to_main"
        ))
    
    return keyboard

def create_faq_keyboard(faq_items):
    """
    Create keyboard for FAQ list
    
    Args:
        faq_items (list): List of FAQ items
        
    Returns:
        InlineKeyboardMarkup: The FAQ keyboard
    """
    keyboard = InlineKeyboardMarkup(row_width=1)
    
    for idx, faq_item in enumerate(faq_items):
        keyboard.add(InlineKeyboardButton(
            text=faq_item.get("question", f"Вопрос {idx+1}"),
            callback_data=f"faq_{idx}"
        ))
    
    keyboard.add(InlineKeyboardButton(text="⬅️ Главное меню", callback_data="back_to_main"))
    
    return keyboard

def create_faq_navigation_keyboard(idx, faq_items_count):
    """
    Create navigation keyboard for FAQ items
    
    Args:
        idx (int): Current FAQ item index
        faq_items_count (int): Total number of FAQ items
        
    Returns:
        InlineKeyboardMarkup: The FAQ navigation keyboard
    """
    keyboard = InlineKeyboardMarkup(row_width=1)
    
    # Navigation buttons for FAQ
    nav_row = []
    if idx > 0:
        nav_row.append(InlineKeyboardButton(text="◀️ Пред.", callback_data=f"faq_{idx-1}"))
    
    nav_row.append(InlineKeyboardButton(text="Назад к FAQ", callback_data="back_to_faq"))
    
    if idx < faq_items_count - 1:
        nav_row.append(InlineKeyboardButton(text="След. ▶️", callback_data=f"faq_{idx+1}"))
    
    keyboard.row(*nav_row)
    keyboard.add(InlineKeyboardButton(text="⬅️ Главное меню", callback_data="back_to_main"))
    
    return keyboard 