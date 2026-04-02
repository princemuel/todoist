import { deleteTask, TaskStatus, updateTask } from "@/actions/crud";

interface Props {
  id: string;
  title: string;
  message: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  rerender: (response: any) => void;
}

export const TaskItem: React.FC<Props> = ({ title, id, message, rerender }) => {
  const sendRequest = async () => {
    if (message === "edit") {
      await updateTask(title, TaskStatus.DONE).then((response) => {
        rerender(response);
      });
    } else {
      await deleteTask(title).then((response) => {
        rerender(response);
      });
    }
  };

  return (
    <li className="itemContainer" id={id}>
      <p>{title}</p>
      <button type="button" className="actionButton" onClick={sendRequest}>
        {message}
      </button>
    </li>
  );
};
