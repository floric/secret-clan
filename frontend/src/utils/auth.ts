import jwtDecode from "jwt-decode";

const ACCESS_TOKEN = "ACCESS_TOKEN";

export const getToken = () => window.localStorage.getItem(ACCESS_TOKEN) || null;

export const saveToken = (token: string) =>
  window.localStorage.setItem(ACCESS_TOKEN, token);

export const clearToken = () => window.localStorage.removeItem(ACCESS_TOKEN);

export type Claims = { sub: string; name: string; game: string };

export const getClaims = (): Claims => {
  const token = getToken();
  if (!token) {
    throw new Error("Token missing");
  }

  return jwtDecode<Claims>(token);
};
