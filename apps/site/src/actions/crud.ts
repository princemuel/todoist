import { TaskURL } from "@/lib/url";
import { del, get, patch, post } from "@/lib/utils";

export const TaskStatus = { PENDING: "PENDING", DONE: "DONE" } as const;

const taskURL = new TaskURL();

export const getAllTasks = async () => {
  return await get<Tasks>(taskURL.url, 200);
};

export const createTask = async (title: string) => {
  const payload: CreateTaskPayload = { title, status: TaskStatus.PENDING };
  return await post<CreateTaskPayload, Tasks>(taskURL.url, payload, 201);
};

export const updateTask = async (id: string, title: string, status: TaskStatus) => {
  const payload: Task = { id, title, status };
  return await patch<Task, Tasks>(taskURL.url, payload, 200);
};

export const deleteTask = async (id: string) => {
  return await del<Task>(taskURL.forId(id), 200);
};
