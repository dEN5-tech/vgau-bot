def format_data_display(data):
    """
    Format data for display in messages
    
    Args:
        data (dict or any): The data to format
        
    Returns:
        str: Formatted text for display
    """
    if isinstance(data, dict):
        result = ""
        
        # Special handling for specialties list
        if "specialties" in data:
            specialties = data.get("specialties", [])
            for specialty in specialties:
                result += f"<b>🎓 {specialty.get('name', '')}</b> ({specialty.get('code', '')})\n"
                if "profile" in specialty:
                    result += f"Профиль: {specialty.get('profile', '')}\n"
                result += "\n"
            return result
            
        # Regular dictionary formatting
        for key, value in data.items():
            # Format key from snake_case to Title Case
            formatted_key = " ".join(word.capitalize() for word in key.split("_"))
            
            if key == "phone":
                result += f"📞 <b>{formatted_key}</b>: {value}\n"
            elif key == "email":
                result += f"✉️ <b>{formatted_key}</b>: {value}\n"
            elif key == "address":
                result += f"📍 <b>{formatted_key}</b>: {value}\n"
            elif key == "hours":
                result += f"🕒 <b>{formatted_key}</b>: {value}\n"
            elif key == "telegram":
                result += f"📱 <b>Telegram</b>: <a href='{value}'>@Agrobioteh37</a>\n"
            elif key == "vk":
                result += f"🌐 <b>ВКонтакте</b>: <a href='{value}'>Группа ВК</a>\n"
            elif key == "ok":
                result += f"🌐 <b>Одноклассники</b>: <a href='{value}'>Группа ОК</a>\n"
            elif key == "contact_page":
                result += f"\n<a href='{value}'>Все контакты на сайте</a>\n"
            else:
                result += f"<b>{formatted_key}</b>: {value}\n"
        
        return result
    else:
        return str(data) 