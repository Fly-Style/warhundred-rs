import {useState} from "react";

export const Chat = () => {
  const [messages, setMessages] = useState([]);
  const [newMessage, setNewMessage] = useState('');

  const handleInputChange = (e) => {
    setNewMessage(e.target.value);
  };

  const handleSendMessage = (e) => {
    if (e.key === 'Enter') {
      sendMessage();
    }
  };

  const sendMessage = () => {
    if (newMessage.trim()) {
      setMessages([...messages, newMessage]);
      setNewMessage('');
    }
  };

  return (
    <div className="cnt chat-parent">
      <div className="chat-handlers"></div>
      <div className="chat-history">
        {messages.map((message, index) => (
          <p key={index} className="message">
            {message}
          </p>
        ))}
      </div>
      <div className="chat-input-wrap">
        <input type="text"
               className="chat-input"
               value={newMessage}
               onChange={handleInputChange}
               onKeyDown={handleSendMessage}
               placeholder="Type your message..."/>
        <input type="submit"
               className="chat-submit"
               value="Send"
               onSubmit={sendMessage}
               placeholder="Send"/>
      </div>
    </div>
  );
}