import { Component, signal, inject, viewChild, ElementRef} from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { ImageService } from './Services/image-service';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  protected readonly title = signal('Aether');
  private imageService = inject(ImageService);
  private fileInput = viewChild<ElementRef<HTMLInputElement>>('fileInput');
  private options = viewChild<ElementRef<HTMLInputElement>>('options')
  imageUrl = signal("");
  convert() : void
  {
    const file = this.fileInput()?.nativeElement.files?.[0];
    const target = this.options()?.nativeElement.value || "";
    if (!file) return;
    this.imageService.convert(file,{target:target}).subscribe((blob)=>{
      this.imageUrl.set(URL.createObjectURL(blob));
    })
  }
}
