from aiogram import types
from aiogram.dispatcher import FSMContext

from utils.logger import log_user_interaction
from modules.states import BotStates
from handlers.search_handlers import process_search

async def register_message_handlers(dp):
    """Register message handlers"""
    dp.register_message_handler(handle_search_query, state=BotStates.search)
    dp.register_message_handler(handle_unknown, state="*")

async def handle_search_query(message: types.Message, state: FSMContext):
    """
    Handle incoming search query
    
    Args:
        message (types.Message): The message object
        state (FSMContext): The current state
    """
    # Delegate to the search handler
    await process_search(message, state)

async def handle_unknown(message: types.Message, state: FSMContext):
    """
    Handle any unknown message
    
    Args:
        message (types.Message): The message object
        state (FSMContext): The current state
    """
    user_id = message.from_user.id
    log_user_interaction(user_id, "unknown_message", {"text": message.text})
    
    current_state = await state.get_state()
    
    if current_state is None:
        await BotStates.main_menu.set()
        await message.reply(
            "Я не понимаю этой команды. Пожалуйста, воспользуйтесь меню или "
            "командой /help для получения списка доступных команд."
        )
    else:
        # If we're in a specific state, notify the user
        state_messages = {
            BotStates.main_menu.state: "Вы находитесь в главном меню. Выберите интересующий вас раздел или воспользуйтесь командой /search для поиска информации.",
            BotStates.search.state: "Вы находитесь в режиме поиска. Введите поисковый запрос или /cancel для отмены.",
        }
        
        msg = state_messages.get(
            current_state, 
            "Я не понимаю этой команды. Воспользуйтесь командой /help для получения списка доступных команд."
        )
        
        await message.reply(msg) 