from aiogram import types
from aiogram.dispatcher import FSMContext

from utils.logger import log_user_interaction
from utils.data_loader import load_bot_data
from keyboards.menu_keyboards import create_main_menu_keyboard, create_faq_keyboard

from modules.states import BotStates
from handlers.search_handlers import process_search

async def register_command_handlers(dp):
    """Register all command handlers"""
    dp.register_message_handler(send_welcome, commands=['start', 'help'])
    dp.register_message_handler(show_menu, commands=['menu'])
    dp.register_message_handler(search_mode, commands=['search'])
    dp.register_message_handler(show_faq, commands=['faq'])
    dp.register_message_handler(handle_unknown_message, state="*")

async def send_welcome(message: types.Message):
    """
    This handler will be called when user sends `/start` or `/help` command
    """
    user_id = message.from_user.id
    log_user_interaction(user_id, "start_command")
    
    welcome_text = (
        "👋 Добро пожаловать в бот приемной комиссии Верхневолжского государственного "
        "агробиотехнологического университета!\n\n"
        "Здесь вы найдете информацию о направлениях обучения, сроках приема документов, "
        "правилах поступления и многое другое.\n\n"
        "Выберите интересующий вас раздел:"
    )
    
    await message.answer(welcome_text, reply_markup=create_main_menu_keyboard())
    await BotStates.main_menu.set()

async def show_menu(message: types.Message):
    """
    Show main menu
    """
    user_id = message.from_user.id
    log_user_interaction(user_id, "menu_command")
    
    await message.answer("Главное меню:", reply_markup=create_main_menu_keyboard())
    await BotStates.main_menu.set()

async def search_mode(message: types.Message):
    """
    Enable search mode
    """
    user_id = message.from_user.id
    log_user_interaction(user_id, "search_command")
    
    await message.answer("Введите ключевое слово для поиска информации:")
    await BotStates.search.set()

async def show_faq(message: types.Message):
    """
    Show FAQ menu
    """
    user_id = message.from_user.id
    log_user_interaction(user_id, "faq_command")
    
    bot_data = load_bot_data()
    faq_items = bot_data.get("faq", [])
    
    if not faq_items:
        await message.answer("FAQ раздел пока не заполнен")
        return
    
    await message.answer("Часто задаваемые вопросы:", reply_markup=create_faq_keyboard(faq_items))

async def handle_unknown_message(message: types.Message, state: FSMContext):
    """
    Handle messages that don't match any other handler
    """
    user_id = message.from_user.id
    text = message.text
    log_user_interaction(user_id, "unknown_message", {"text": text})
    
    # Check if this looks like a search query
    if len(text) > 3:
        # Treat as search query
        await process_search(message, state)
    else:
        await message.answer(
            "Я не понимаю этот запрос. Воспользуйтесь меню или отправьте запрос для поиска.",
            reply_markup=create_main_menu_keyboard()
        ) 