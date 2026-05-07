import { Injectable,inject } from '@angular/core';
import { BaseService } from './base-service';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class ImageService 
{
  private _baseService = inject(BaseService);

  convert(image:any,options:imageOptions) : Observable<any>
  {
    const formData = new FormData();
  
    formData.append('options', new Blob([JSON.stringify(options)]));
    formData.append('file', image);

    return this._baseService.post<Blob>("image/convert",formData,{ responseType: 'blob' })
  }
}
interface imageOptions
{
  target:string
}
