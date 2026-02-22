import { useEffect, useState } from "react";

function App() {
  const [messages, setMessages] = useState([]);
  const [error, setError] = useState("");
  const [text, setText] = useState("");

  useEffect(() => {
    fetch("http://localhost:8080/messages")
      .then((res) => {
        if (!res.ok) throw new Error("Failed to fetch messages");
        return res.json();
      })
      .then(setMessages)
      .catch((err) => setError(err.message));
  }, []);

  const handleSubmit = async (e) => {
    e.preventDefault();
    const res = await fetch("http://localhost:8080/message", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ text }),
    });
    const newMsg = await res.json();
    setMessages([...messages, newMsg]);
    setText("");
  };

  return (
    <div style={{ padding: "2rem" }}>
      <h1>Messages</h1>
      <form onSubmit={handleSubmit}>
        <input value={text} onChange={(e) => setText(e.target.value)} />
        <button type="submit">Add</button>
      </form>
      {error && <p style={{ color: "red" }}>{error}</p>}
      {messages.length === 0 ? (
        <p>No messages yet</p>
      ) : (
        <ul>
          {messages.map((m) => (
            <li key={m.id}>{m.text}</li>
          ))}
        </ul>
      )}
    </div>
  );
}

export default App;
