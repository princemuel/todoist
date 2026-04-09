const DEFAULT_HEADERS = {
  token: "jwt",
  "Content-Type": "application/json"
};

async function request<X>(promise: Promise<Response>, status_code: number) {
  let response: Response;
  try {
    response = await promise;
  } catch {
    return {
      status: 500,
      error: "Network Error",
      data: null
    };
  }

  let data: X;
  try {
    data = (await response.json()) as X;
  } catch {
    return {
      status: response.status,
      error: "Failed to parse response",
      data: null
    };
  }

  if (response.status === status_code) {
    return { status: response.status, data };
  } else {
    return {
      status: response.status,
      error: `expected status ${status_code} got ${response.status}`,
      data
    };
  }
}

export async function get<X>(url: URL, status_code: number) {
  return request<X>(fetch(url, { headers: DEFAULT_HEADERS }), status_code);
}

export async function post<T, X>(url: URL, payload: T, status_code: number) {
  return request<X>(
    fetch(url, { method: "POST", headers: DEFAULT_HEADERS, body: JSON.stringify(payload) }),
    status_code
  );
}

export async function put<T, X>(url: URL, payload: T, status_code: number) {
  return request<X>(
    fetch(url, { method: "PUT", headers: DEFAULT_HEADERS, body: JSON.stringify(payload) }),
    status_code
  );
}

export async function patch<T, X>(url: URL, payload: T, status_code: number) {
  return request<X>(
    fetch(url, { method: "PATCH", headers: DEFAULT_HEADERS, body: JSON.stringify(payload) }),
    status_code
  );
}

export async function del<X>(url: URL, status_code: number) {
  return request<X>(fetch(url, { method: "DELETE", headers: DEFAULT_HEADERS }), status_code);
}
