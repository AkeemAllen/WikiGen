import {
  getToastStore,
  initializeStores,
  type ToastSettings,
} from "@skeletonlabs/skeleton";

export enum ToastType {
  SUCCESS = "success",
  INFO = "info",
  ERROR = "error",
}

export function getToastSettings(
  type: ToastType,
  message: string,
): ToastSettings {
  switch (type) {
    case ToastType.SUCCESS:
      return {
        message,
        timeout: 3000,
        background: "bg-white",
        classes: "border-l-8 border-green-500",
      };
      break;
    case ToastType.INFO:
      return {
        message,
        background: "bg-white",
        classes: "border-l-8 border-blue-500",
      };
      break;
    case ToastType.ERROR:
      return {
        message,
        autohide: false,
        background: "bg-white",
        classes: "border-l-8 border-red-500",
      };
      break;
  }
}
