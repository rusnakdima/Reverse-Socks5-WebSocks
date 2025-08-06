export interface ResponseModel {
  status: ResponseStatus;
  message: string;
  data: any;
}

export enum ResponseStatus {
  Success = 'Success',
  Info = 'Info',
  Warning = 'Warning',
  Error = 'Error',
}
