import { HttpClient } from '@angular/common/http';
import { Injectable,inject } from '@angular/core';

@Injectable({
  providedIn: 'root',
})
export class BaseService 
{
  private readonly baseUrl = '/api/';
  private _http = inject(HttpClient);

  post<T>(path:string,body:any,options:any = {})
  {
    return this._http.post<T>(`${this.baseUrl}${path}`, body, options);
  }
}
