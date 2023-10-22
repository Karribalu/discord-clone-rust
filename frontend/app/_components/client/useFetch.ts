import { useRouter } from "next/router";
function useFetch() {
  const router = useRouter();

  return {
    get: request("GET"),
  };
  function request(method: string) {
    return (url: string, body?: any) => {
      const requestOptions: any = {
        method,
      };
      if (body) {
        requestOptions.headers = { "Content-Type": "application/json" };
        requestOptions.body = JSON.stringify(body);
      }
      return fetch(url, requestOptions).then(handleResponse);
    };
  }

  async function handleResponse(response: any) {
    const isJson = response.headers
      ?.get("Content-Type")
      ?.includes("application/json");

    const data = isJson ? response.json() : null;
    if (!response.ok) {
      if (response.status === 401) {
        router.push("/account/login");
      }
      const error = (data && data.message) || response.statusText;
      return Promise.reject(error);
    }

    return data;
  }
}
