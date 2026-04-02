import { getAllTasks } from "@/actions/crud";
import { Form } from "@/components/form";
import { TaskItem } from "@/components/task-item";
import { useEffect, useState } from "react";

import init, { rust_generate_button_text } from "../interface/pkg/interface.js";

export default function App() {
  const [data, setData] = useState<Tasks | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isWasmReady, setIsWasmReady] = useState(false);
  const [rustButtonText, setRustButtonText] = useState<((input: string) => string) | null>(null);

  useEffect(() => {
    init()
      .then(() => {
        setRustButtonText(() => rust_generate_button_text);
        setIsWasmReady(true);
      })
      .catch((err) => {
        console.error("Failed to initialize WASM module:", err);
        setError("Failed to load WASM module");
      });
  }, []);

  useEffect(() => {
    const fetchData = async () => {
      const response = await getAllTasks();
      if (response.error) {
        setError(response.error);
      } else if (response.data) {
        setData(response.data);
      }
    };

    if (isWasmReady) fetchData();
  }, [isWasmReady]);

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  function rerender(response: any) {
    if (response.error) {
      alert(JSON.stringify(response));
      return;
    } else if (response.data) {
      setData(response.data);
      setError(null);
    } else {
      setError("Unknown error");
    }
  }

  if (error) {
    return <div style={{ color: "red" }}>Error: {error}</div>;
  } else if (!data) {
    return <div>Loading...</div>;
  }

  return (
    <div className="App">
      <main className="mainContainer">
        <div className="header">
          <p>complete tasks: {data.done.length}</p>
          <p>pending tasks: {data.pending.length}</p>
        </div>

        <h2>Pending Items</h2>
        <ul>
          {data.pending.map((item) => (
            <TaskItem
              key={item.id}
              id={item.id}
              title={item.title}
              message={rustButtonText ? rustButtonText(item.status) : item.status}
              rerender={rerender}
            />
          ))}
        </ul>

        <h2>Done Items</h2>
        <ul>
          {data.done.map((item) => (
            <TaskItem
              key={item.id}
              id={item.id}
              title={item.title}
              message={rustButtonText ? rustButtonText(item.status) : item.status}
              rerender={rerender}
            />
          ))}
        </ul>

        <Form rerender={rerender} />
      </main>
    </div>
  );
}
