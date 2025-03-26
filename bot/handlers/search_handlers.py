from aiogram import types
from aiogram.dispatcher import FSMContext
from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton

from utils.logger import log_user_interaction
from utils.data_loader import load_bot_data
from keyboards.menu_keyboards import create_main_menu_keyboard
from modules.states import BotStates

async def register_search_handlers(dp):
    """Register search handlers"""
    dp.register_message_handler(process_search, state=BotStates.search)

async def process_search(message: types.Message, state: FSMContext):
    """
    Process search query
    
    Args:
        message (types.Message): The message with the search query
        state (FSMContext): The current state
    """
    user_id = message.from_user.id
    search_query = message.text.lower()
    log_user_interaction(user_id, "search", {"query": search_query})
    
    # Implement search logic based on keywords
    bot_data = load_bot_data()
    search_results = []
    
    # Search through main menu items
    for item in bot_data.get("main_menu", []):
        item_text = item.get("text", "").lower()
        if search_query in item_text:
            search_results.append({
                "text": item.get("text"),
                "callback_data": item.get("callback_data")
            })
        
        # Search in descriptions
        description = item.get("description", "").lower()
        if search_query in description:
            search_results.append({
                "text": item.get("text"),
                "callback_data": item.get("callback_data")
            })
        
        # Search in submenus
        for submenu_item in item.get("submenu", []):
            submenu_text = submenu_item.get("text", "").lower()
            if search_query in submenu_text:
                search_results.append({
                    "text": submenu_item.get("text"),
                    "callback_data": submenu_item.get("callback_data")
                })
            
            # Search in submenu descriptions
            submenu_description = submenu_item.get("description", "").lower()
            if search_query in submenu_description:
                search_results.append({
                    "text": submenu_item.get("text"),
                    "callback_data": submenu_item.get("callback_data")
                })
                
            # Search in documents
            for doc in submenu_item.get("documents", []):
                doc_text = doc.get("text", "").lower()
                if search_query in doc_text:
                    if "url" in doc:
                        search_results.append({
                            "text": f"📄 {doc.get('text')}",
                            "url": doc.get("url")
                        })
                    else:
                        search_results.append({
                            "text": f"📄 {doc.get('text')}",
                            "callback_data": doc.get("callback_data")
                        })
    
    # Search in FAQ
    for idx, faq_item in enumerate(bot_data.get("faq", [])):
        question = faq_item.get("question", "").lower()
        answer = faq_item.get("answer", "").lower()
        
        if search_query in question or search_query in answer:
            search_results.append({
                "text": f"❓ {faq_item.get('question')}",
                "callback_data": f"faq_{idx}"
            })
    
    if search_results:
        keyboard = InlineKeyboardMarkup(row_width=1)
        for result in search_results:
            if "url" in result:
                keyboard.add(InlineKeyboardButton(
                    text=result.get("text"),
                    url=result.get("url")
                ))
            else:
                keyboard.add(InlineKeyboardButton(
                    text=result.get("text"),
                    callback_data=result.get("callback_data")
                ))
        keyboard.add(InlineKeyboardButton(text="⬅️ В главное меню", callback_data="back_to_main"))
        
        await message.answer(f"Результаты поиска по запросу '{search_query}':", reply_markup=keyboard)
    else:
        keyboard = InlineKeyboardMarkup()
        keyboard.add(InlineKeyboardButton(text="⬅️ В главное меню", callback_data="back_to_main"))
        
        await message.answer(
            f"По запросу '{search_query}' ничего не найдено. Попробуйте другие ключевые слова или "
            f"воспользуйтесь меню для навигации.",
            reply_markup=keyboard
        )
    
    await BotStates.main_menu.set() 