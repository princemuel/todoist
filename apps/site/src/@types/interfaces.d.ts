interface Task {
  title: string;
  status: TaskStatus;
}

interface Tasks {
  pending: Task[];
  done: Task[];
}

const TaskStatus = {
  PENDING: "PENDING",
  DONE: "DONE"
} as const;

type TaskStatus = (typeof TaskStatus)[keyof typeof TaskStatus];
