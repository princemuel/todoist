export class TaskURL {
  #baseUrl: URL;
  url: URL;

  constructor() {
    this.#baseUrl = TaskURL.#getBaseUrl();
    this.url = new URL("/api/v1/tasks", this.#baseUrl);
  }

  forId(id: string): URL {
    return new URL(`${this.url.pathname}/${id}`, this.#baseUrl);
  }

  static #getBaseUrl() {
    const url = window.location.href;
    if (url.includes("http://localhost:3000/")) return new URL("http://0.0.0.0:8001");
    return new URL(url);
  }
}
