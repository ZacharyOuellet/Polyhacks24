import { Component, OnInit, Input } from '@angular/core';
import { NodeIdObj } from 'src/app/interfaces';
import { NoeuComponent } from '../noeu/noeu.component';
import * as d3 from 'd3';
import { GraphService } from 'src/app/service/graph.service';
//https://blog.logrocket.com/data-visualization-angular-d3-js/#setting-up-angular-d3

@Component({
  selector: 'app-svg-map',
  standalone: true,
  templateUrl: './svg-map.component.html',
  styleUrls: ['./svg-map.component.scss']
})

export class SvgMapComponent implements OnInit {
    nodes : NodeIdObj[] = [];
    @Input() links : number[][];
    
    ratioX : number = 890-50;
    ratioY : number = 480-50;
    nodeId : number = -1;
    color: string[] = ['red','red','red']
    formData = {
      condDep: '',
      condArr: '',
      passDep: '',
      passArr: ''
    };
    constructor(private graphService: GraphService){
      let i = 0;
      this.graphService.graph.nodes.forEach(node => {
        let element = { id:i, x:node.x, y: node.y};
       
        this.nodes.push(element);
        i++;
      });
      
      this.links = [[0,1],[1,2],[2,0]]
    }

    ngOnInit(): void {
      const svg = d3.select("#map")
      .append('svg')
      .attr("preserveAspectRatio", "xMinYMin meet")
      .attr("viewBox", "0 0 960 500")
      .attr("transform",
        `translate(${50}, ${50})`);

      this.links.forEach(link => {
        const source = this.nodes[link[0]];
        const target = this.nodes[link[1]];
        svg.append("line")
          .attr("x1", source.x*this.ratioX)
          .attr("y1", source.y*this.ratioY)
          .attr("x2", target.x*this.ratioX)
          .attr("y2", target.y*this.ratioY)
          .style("stroke", "black")
          .style("stroke-width", 2);
      });
      
      const self = this; // store reference to 'this' context
    svg.selectAll('circle')
      .data(this.nodes)
      .enter()
      .append('circle')
      .attr('cx', d => d.x*this.ratioX)
      .attr('cy', d => d.y*this.ratioY)
      .attr('r', 10)
      .attr('id', d => d.id) 
      .style('fill', 'red')
      .on('click', function() {
        console.log(this.id);
        self.color[parseInt(this.id)] = 'blue';
        self.onDotClick(parseInt(this.id));
      });

      svg.selectAll('text')
      .data(this.nodes)
      .enter()
      .append('text')
      .attr('x', d => d.x * this.ratioX)
      .attr('y', d => d.y * this.ratioY)
      .text(d => d.id)
      .attr('text-anchor', 'middle')
      .attr('dy', '0.3em') // Adjust vertical alignment
      .style('fill', 'white')
      .style('font-size', '12px');
  }
  

  onDotClick(nodeid: number): void {
    const svg = d3.select("#map")
      .append('svg')
      .attr("preserveAspectRatio", "xMinYMin meet")
      .attr("viewBox", "0 0 960 500")
      .attr("transform",
        `translate(${50}, ${50})`);

      this.links.forEach(link => {
        const source = this.nodes[link[0]];
        const target = this.nodes[link[1]];
        svg.append("line")
          .attr("x1", source.x*this.ratioX)
          .attr("y1", source.y*this.ratioY)
          .attr("x2", target.x*this.ratioX)
          .attr("y2", target.y*this.ratioY)
          .style("stroke", "black")
          .style("stroke-width", 2);
      });
      
      const self = this; // store reference to 'this' context
      svg.selectAll('circle')
      .data(this.nodes)
      .enter()
      .append('circle')
      .attr('cx', d => d.x*this.ratioX)
      .attr('cy', d => d.y*this.ratioY)
      .attr('r', 10)
      .attr('id', d => d.id) 
      .style('fill', d => this.color[d.id])
      .on('click', function() {
        console.log(this.id);
        self.color[parseInt(this.id)] = 'blue';
        self.onDotClick(parseInt(this.id));
      });
  }
  soumettre(a:string,b:string,c:string,d:string){
    const cd = a;
    const ca = b;
    const pd = c;
    const pa = d;
    //send to server
    //generate
  }
}

