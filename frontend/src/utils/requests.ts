import { getToken } from "./auth";

export const sendRequest = async <R>(
  url: string,
  method: "GET" | "POST" | "PUT",
  payload?: object
): Promise<R | null> => {
  const token = getToken();
  const res = await fetch(url, {
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(payload ? { "Content-Type": "application/json" } : {}),
    },
    body: JSON.stringify(payload),
    method,
  });
  if (!res.ok) {
    return null;
  }
  return (await res.json()) as R;
};
