import { getAllTasks } from "@/actions/crud";
import { useEffect, useId, useState } from "react";
import { TaskItem } from "@/components/task-item";
import { Form } from "@/components/form";

export default function App() {
  const [data, setData] = useState<Tasks | null>(null);
  const [error, setError] = useState<string | null>(null);
  const id = useId();

  useEffect(() => {
    const fetchData = async () => {
      const response = await getAllTasks();
      if (response.error) {
        setError(response.error);
      } else if (response.data) {
        setData(response.data);
      }
    };
    fetchData();
  }, []);

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
        <div>
          {data.pending.map((item) => (
            <TaskItem
              key={id + item.title + item.status}
              title={item.title}
              status={item.status}
              id={id + item.title}
              rerender={rerender}
            />
          ))}
        </div>

        <h2>Done Items</h2>
        <div>
          {data.done.map((item) => (
            <TaskItem
              key={id + item.title + item.status}
              title={item.title}
              status={item.status}
              id={id + item.title}
              rerender={rerender}
            />
          ))}
        </div>

        <Form rerender={rerender} />
      </main>
    </div>
  );
}
