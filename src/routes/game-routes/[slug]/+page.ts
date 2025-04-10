export function load({ params }: any) {
  return {
    title: params.slug,
  };
}

export function entries() {
  return [{ slug: "hello-world" }];
}

export const prerender = true;
