import { useQuery } from "react-query";

const fetchPools = async (limit = 10) => {
  const parsed = await (
    await fetch("https://jsonplaceholder.typicode.com/posts")
  ).json();
  return parsed;
};

const usePools = (limit) => {
  return useQuery(["posts", limit], () => fetchPosts(limit));
};

export { usePools, fetchPools };
