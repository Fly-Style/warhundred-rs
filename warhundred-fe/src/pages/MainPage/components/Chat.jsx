import {useState, useEffect, useRef} from "react";
import PropTypes from 'prop-types';

// Test data for development and testing
const testMessages = [
  {id: 1, sender: "System", content: "Welcome to the chat!", timestamp: "10:00"},
  {id: 2, sender: "Warrior123", content: "Hello everyone!", timestamp: "10:05"},
  {id: 3, sender: "ArcherQueen", content: "Hi there!", timestamp: "10:07"},
  {id: 4, sender: "DarkWizard", content: "Anyone want to team up for a raid?", timestamp: "10:10"},
  {id: 5, sender: "HealerGirl", content: "I can join as a healer", timestamp: "10:12"}
];

export const Chat = ({useTestData = false}) => {
  const [messages, setMessages] = useState(useTestData ? testMessages : []);
  const [newMessage, setNewMessage] = useState('');
  const [error, setError] = useState(null);
  const [isLoading, setIsLoading] = useState(!useTestData);
  const chatHistoryRef = useRef(null);

  useEffect(() => {
    // Skip API calls if using test data
    if (useTestData) {
      setIsLoading(false);
      return;
    }

    // Function to fetch chat messages
    const fetchChatMessages = async () => {
      try {
        setError(null);
        setIsLoading(true);

        // In a real app, this would be an API call
        // For now, we'll just simulate a delay and then set some messages
        setTimeout(() => {
          setMessages([
            {id: 1, sender: "System", content: "Welcome to the chat!", timestamp: "10:00"}
          ]);
          setIsLoading(false);
        }, 1000);
      } catch (err) {
        setError("Error loading chat messages");
        setIsLoading(false);
      }
    };

    // Initial fetch when component mounts
    fetchChatMessages();

    // Setup interval to fetch messages every few seconds
    const intervalId = setInterval(fetchChatMessages, 10000);

    // Clean up function to clear an interval when the component unmounts
    return () => {
      clearInterval(intervalId);
    };
  }, [useTestData]);

  // Auto-scroll to bottom when messages change
  useEffect(() => {
    if (chatHistoryRef.current) {
      chatHistoryRef.current.scrollTop = chatHistoryRef.current.scrollHeight;
    }
  }, [messages]);

  const handleInputChange = (e) => {
    setNewMessage(e.target.value);
  };

  const handleSendMessage = (e) => {
    if (e.key === 'Enter') {
      sendMessage();
    }
  };

  const handleSubmitClick = () => {
    sendMessage();
  };

  const sendMessage = () => {
    if (newMessage.trim()) {
      const newMsg = {
        id: messages.length + 1,
        sender: "You",
        content: newMessage,
        timestamp: new Date().toLocaleTimeString([], {hour: '2-digit', minute: '2-digit'})
      };
      setMessages([...messages, newMsg]);
      setNewMessage('');
    }
  };

  return (
    <div className="chat-wrapper">
      <div className="chat-header">
        <h3>Chat</h3>
        <span className="message-count">{messages.length} messages</span>
      </div>

      {isLoading && <div className="loading-indicator">Loading messages...</div>}
      {error && <div className="error-message">{error}</div>}

      <div className="chat-history" ref={chatHistoryRef}>
        {messages.length > 0 ? (
          messages.map((message) => (
            <div key={message.id}
                 className="message-box message-incoming">
              <span className="message-timestamp">{message.timestamp}</span>

              <span className="message-sender"><b>{message.sender}</b>:</span>
              <span className="message-content">{message.content}</span>
            </div>
          ))
        ) : (
          !isLoading && !error && <div className="empty-state">No messages yet</div>
        )}
      </div>

      <div className="chat-input-wrap">
        <input
          type="text"
          className="chat-input"
          value={newMessage}
          onChange={handleInputChange}
          onKeyDown={handleSendMessage}
          placeholder="Type your message..."
          disabled={isLoading}
        />
        <button
          className="chat-submit"
          onClick={handleSubmitClick}
          disabled={isLoading || !newMessage.trim()}
        >
          Send
        </button>
      </div>
    </div>
  );
};

Chat.propTypes = {
  useTestData: PropTypes.bool
};
