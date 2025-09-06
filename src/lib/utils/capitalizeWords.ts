export default function capitalizeWords(str: string | null): string {
  if (str === null) return "";
  return str
    .split(/[-_]/)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(" ");
}
