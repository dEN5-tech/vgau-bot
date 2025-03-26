from aiogram import types, Bot
from aiogram.dispatcher import FSMContext
from aiogram.types import InlineKeyboardMarkup, InlineKeyboardButton

from utils.logger import log_user_interaction
from utils.data_loader import get_menu_item_by_callback, load_bot_data
from utils.formatters import format_data_display
from keyboards.menu_keyboards import (
    create_main_menu_keyboard, 
    create_submenu_keyboard, 
    create_sub_submenu_keyboard,
    create_document_keyboard,
    create_faq_navigation_keyboard
)
from modules.states import BotStates
from handlers.command_handlers import show_faq

# Initialization to be done in application bootstrap
bot = None

def init_bot(bot_instance):
    """Initialize the bot instance for this module"""
    global bot
    bot = bot_instance

async def register_callback_handlers(dp):
    """Register callback query handlers"""
    dp.register_callback_query_handler(process_callback, state="*")
    dp.register_callback_query_handler(process_faq_selection, lambda c: c.data.startswith('faq_'), state="*")
    dp.register_callback_query_handler(back_to_faq, lambda c: c.data == "back_to_faq", state="*")

async def process_callback(callback_query: types.CallbackQuery, state: FSMContext):
    """
    Process callback query
    
    Args:
        callback_query (types.CallbackQuery): The callback query
        state (FSMContext): The current state
    """
    user_id = callback_query.from_user.id
    callback_data = callback_query.data
    log_user_interaction(user_id, "callback", {"data": callback_data})
    
    # Skip FAQ callbacks as they are handled separately
    if callback_data.startswith("faq_") or callback_data == "back_to_faq":
        return
    
    # Handle back to main menu
    if callback_data == "back_to_main":
        await bot.edit_message_text(
            chat_id=callback_query.message.chat.id,
            message_id=callback_query.message.message_id,
            text="Главное меню:",
            reply_markup=create_main_menu_keyboard()
        )
        await BotStates.main_menu.set()
        return
    
    # Handle back to submenu
    if callback_data.startswith("back_to_"):
        parent_menu = callback_data.replace("back_to_", "")
        parent_item = get_menu_item_by_callback(parent_menu)
        
        if parent_item:
            await bot.edit_message_text(
                chat_id=callback_query.message.chat.id,
                message_id=callback_query.message.message_id,
                text=parent_item.get("text", "Подменю:"),
                reply_markup=create_submenu_keyboard(parent_menu)
            )
        else:
            await bot.edit_message_text(
                chat_id=callback_query.message.chat.id,
                message_id=callback_query.message.message_id,
                text="Главное меню:",
                reply_markup=create_main_menu_keyboard()
            )
        
        return
    
    # Handle pagination for documents
    if callback_data.startswith("doc_page_"):
        parts = callback_data.split("_")
        parent_callback = parts[3]
        page_num = int(parts[4])
        await show_documents_page(callback_query, parent_callback, page_num)
        return
    
    # Process other callbacks
    menu_item = get_menu_item_by_callback(callback_data)
    
    if not menu_item:
        await bot.answer_callback_query(callback_query.id, text="Информация не найдена")
        return
    
    # Check if this is a main menu item with submenu
    for item in load_bot_data().get("main_menu", []):
        if item.get("callback_data") == callback_data:
            if "submenu" in item:
                await bot.edit_message_text(
                    chat_id=callback_query.message.chat.id,
                    message_id=callback_query.message.message_id,
                    text=item.get("text", "Выберите раздел:"),
                    reply_markup=create_submenu_keyboard(callback_data)
                )
                return
            elif "url" in item:
                keyboard = InlineKeyboardMarkup()
                keyboard.add(InlineKeyboardButton(
                    text="🔗 Открыть ссылку",
                    url=item['url']
                ))
                keyboard.add(InlineKeyboardButton(
                    text="⬅️ Назад",
                    callback_data="back_to_main"
                ))
                
                description = item.get("description", "")
                await bot.edit_message_text(
                    chat_id=callback_query.message.chat.id,
                    message_id=callback_query.message.message_id,
                    text=f"{item.get('text')}\n\n{description}",
                    reply_markup=keyboard
                )
                return
            elif "data" in item:
                text = format_data_display(item["data"])
                await bot.edit_message_text(
                    chat_id=callback_query.message.chat.id,
                    message_id=callback_query.message.message_id,
                    text=text,
                    reply_markup=InlineKeyboardMarkup().add(
                        InlineKeyboardButton(text="⬅️ Назад", callback_data="back_to_main")
                    ),
                    parse_mode="HTML"
                )
                return
    
    # Check if this is a submenu item
    for main_item in load_bot_data().get("main_menu", []):
        for submenu_item in main_item.get("submenu", []):
            if submenu_item.get("callback_data") == callback_data:
                # Handle documents array if present
                if "documents" in submenu_item:
                    await show_documents_page(callback_query, callback_data, 1, main_item.get("callback_data"))
                    return
                elif "submenu" in submenu_item:
                    # This is a submenu with its own submenu
                    await bot.edit_message_text(
                        chat_id=callback_query.message.chat.id,
                        message_id=callback_query.message.message_id,
                        text=submenu_item.get("text", "Выберите подраздел:"),
                        reply_markup=create_sub_submenu_keyboard(main_item.get("callback_data"), callback_data)
                    )
                    return
                elif "url" in submenu_item:
                    # This is a submenu with URL
                    keyboard = InlineKeyboardMarkup()
                    keyboard.add(InlineKeyboardButton(
                        text="🔗 Открыть ссылку",
                        url=submenu_item['url']
                    ))
                    keyboard.add(InlineKeyboardButton(
                        text="⬅️ Назад",
                        callback_data=f"back_to_{main_item.get('callback_data')}"
                    ))
                    
                    description = submenu_item.get("description", "")
                    await bot.edit_message_text(
                        chat_id=callback_query.message.chat.id,
                        message_id=callback_query.message.message_id,
                        text=f"{submenu_item.get('text')}\n\n{description}",
                        reply_markup=keyboard
                    )
                    return
                elif "text_content" in submenu_item:
                    # This is a submenu with text content
                    keyboard = InlineKeyboardMarkup()
                    keyboard.add(InlineKeyboardButton(
                        text="⬅️ Назад",
                        callback_data=f"back_to_{main_item.get('callback_data')}"
                    ))
                    
                    await bot.edit_message_text(
                        chat_id=callback_query.message.chat.id,
                        message_id=callback_query.message.message_id,
                        text=f"{submenu_item.get('text')}\n\n{submenu_item.get('text_content')}",
                        reply_markup=keyboard
                    )
                    return
                elif "description" in submenu_item:
                    # This is a submenu with description
                    keyboard = InlineKeyboardMarkup()
                    keyboard.add(InlineKeyboardButton(
                        text="⬅️ Назад",
                        callback_data=f"back_to_{main_item.get('callback_data')}"
                    ))
                    
                    await bot.edit_message_text(
                        chat_id=callback_query.message.chat.id,
                        message_id=callback_query.message.message_id,
                        text=f"{submenu_item.get('text')}\n\n{submenu_item.get('description')}",
                        reply_markup=keyboard
                    )
                    return
                elif "data" in submenu_item:
                    # This is a submenu with data to display
                    text = f"{submenu_item.get('text')}\n\n"
                    text += format_data_display(submenu_item["data"])
                    
                    keyboard = InlineKeyboardMarkup()
                    keyboard.add(InlineKeyboardButton(
                        text="⬅️ Назад",
                        callback_data=f"back_to_{main_item.get('callback_data')}"
                    ))
                    
                    await bot.edit_message_text(
                        chat_id=callback_query.message.chat.id,
                        message_id=callback_query.message.message_id,
                        text=text,
                        reply_markup=keyboard,
                        parse_mode="HTML"
                    )
                    return
    
    # Default fallback
    await bot.answer_callback_query(callback_query.id, text="Раздел в разработке")

async def show_documents_page(callback_query, parent_callback, page_num=1, parent_menu_callback=None):
    """
    Show paginated document list
    
    Args:
        callback_query (types.CallbackQuery): The callback query
        parent_callback (str): The callback data of the parent item
        page_num (int, optional): The current page number. Defaults to 1.
        parent_menu_callback (str, optional): The callback data of the parent menu. Defaults to None.
    """
    item = get_menu_item_by_callback(parent_callback)
    if not item or "documents" not in item:
        await bot.answer_callback_query(callback_query.id, text="Документы не найдены")
        return
    
    documents = item.get("documents", [])
    
    # Create keyboard with document buttons
    keyboard = create_document_keyboard(
        documents, 
        parent_callback, 
        page_num, 
        parent_menu_callback
    )
    
    # Send or edit message
    title = item.get("text", "Документы")
    description = item.get("description", "")
    
    message_text = f"{title}\n\n{description}" if description else title
    
    await bot.edit_message_text(
        chat_id=callback_query.message.chat.id,
        message_id=callback_query.message.message_id,
        text=message_text,
        reply_markup=keyboard
    )

async def process_faq_selection(callback_query: types.CallbackQuery, state: FSMContext):
    """
    Handle FAQ item selection
    
    Args:
        callback_query (types.CallbackQuery): The callback query
        state (FSMContext): The current state
    """
    user_id = callback_query.from_user.id
    idx = int(callback_query.data.split('_')[1])
    log_user_interaction(user_id, "faq_item", {"index": idx})
    
    bot_data = load_bot_data()
    faq_items = bot_data.get("faq", [])
    
    if idx < 0 or idx >= len(faq_items):
        await bot.answer_callback_query(callback_query.id, text="Вопрос не найден")
        return
    
    faq_item = faq_items[idx]
    question = faq_item.get("question", "")
    answer = faq_item.get("answer", "")
    
    keyboard = create_faq_navigation_keyboard(idx, len(faq_items))
    
    await bot.edit_message_text(
        chat_id=callback_query.message.chat.id,
        message_id=callback_query.message.message_id,
        text=f"<b>❓ {question}</b>\n\n{answer}",
        reply_markup=keyboard,
        parse_mode="HTML"
    )

async def back_to_faq(callback_query: types.CallbackQuery, state: FSMContext):
    """
    Handle back to FAQ button
    
    Args:
        callback_query (types.CallbackQuery): The callback query
        state (FSMContext): The current state
    """
    await show_faq(callback_query.message)
    await bot.delete_message(
        chat_id=callback_query.message.chat.id,
        message_id=callback_query.message.message_id
    )
    await bot.answer_callback_query(callback_query.id) 