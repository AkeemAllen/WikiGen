export function cloneDeep(param: any): any {
  if (typeof param === "string") {
    return param;
  } else if (typeof param === "number") {
    return param;
  } else if (typeof param === "boolean") {
    return param;
  } else if (Array.isArray(param)) {
    return param.map((par) => cloneDeep(par));
  } else if (typeof param === "object") {
    // null is an object, so we need to check for it first
    if (param === null) return null;

    const newObj: any = {};
    Object.keys(param).forEach((key) => {
      const value = cloneDeep(param[key]);
      newObj[key] = value;
    });
    return newObj;
  }
}
