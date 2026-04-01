import { deleteTask, updateTask } from "@/actions/crud";
import { TaskStatus } from "@/actions/crud";

interface Props {
  id: string;
  title: string;
  status: string;
  rerender: (response: any) => void;
}

export const TaskItem: React.FC<Props> = ({ title, status, id, rerender }) => {
  const buttonText = status === "PENDING" ? "edit" : "delete";

  const sendRequest = async () => {
    if (buttonText === "edit") {
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
    <article className="itemContainer" id={id}>
      <p>{title}</p>
      <button type="button" className="actionButton" onClick={sendRequest}>
        {buttonText}
      </button>
    </article>
  );
};
