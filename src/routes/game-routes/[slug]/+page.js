export function load({ params }) {
  return {
    title: params.slug,
  };
}

export function entries() {
  return [{ slug: "hello-world" }];
}

export const prerender = true;
