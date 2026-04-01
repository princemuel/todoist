import { createTask } from "@/actions/crud";
import { useState } from "react";

interface Props {
  rerender: (response: any) => void;
}

export const Form: React.FC<Props> = ({ rerender }) => {
  const [title, setTitle] = useState<string>("");

  const handleTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setTitle(e.target.value);
  };
  const createItem = async () => {
    await createTask(title).then((r) => {
      setTitle("");
      rerender(r);
    });
  };
  return (
    <div className="inputContainer">
      <input
        type="text"
        id="name"
        placeholder="create a task"
        value={title}
        onChange={handleTitleChange}
      />
      <button type="button" className="actionButton" id="create-button" onClick={createItem}>
        Create
      </button>
    </div>
  );
};
