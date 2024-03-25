export function capitalize(str: string): string {
  const firstLetter = str[0].toUpperCase();
  return firstLetter + str.slice(1);
}