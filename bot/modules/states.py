from aiogram.dispatcher.filters.state import State, StatesGroup

class BotStates(StatesGroup):
    """Bot states for FSM (Finite State Machine)"""
    main_menu = State()  # Main menu state
    search = State()     # Search mode state 