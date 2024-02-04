import { Component } from '@angular/core';
import { SvgMapComponent } from 'src/app/components/svg-map/svg-map.component';

@Component({
  selector: 'app-main-page',
  standalone: true,
  imports: [SvgMapComponent],
  templateUrl: './main-page.component.html',
  styleUrls: ['./main-page.component.scss']
})
export class MainPageComponent {

}
