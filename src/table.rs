use piston_window::*;
use rand;
use rand::distributions::{IndependentSample, Range};
use mine::Mine;
use piston_window::character::CharacterCache;

use std::f64;
use std;

pub struct Table{
    matrix: Vec<Vec<Mine>>,
    radio: f64,
}

impl Table {
    pub fn new(width: usize, height: usize) -> Self{
        let mut matrix: Vec<Vec<Mine>> = Vec::new();
        for i in 0..height {
            let mut m: Vec<Mine> = Vec::new();
            matrix.push(m);
            for j in 0..width/2 {
                if i % 2 == 1 && j == (height-1){
                    matrix[i].push(Mine::OutOfTable);
                }else{
                    matrix[i].push(Mine::HexCell(false));
                }
                if i% 2 == 0 && j == 0 && width % 2 == 1{
                    matrix[i].push(Mine::HexCell(false));
                }
            }
        }
        let mut table = Table{
            matrix: matrix,
            radio: 0.0
        };
        table.random();
        table

    }

    fn random(&mut self){
        // 10 por ciento de probabilidades
        for i in 0..self.matrix.len(){
            for j in 0..self.matrix[i].len() {
                if Self::pick(0,10) == 0 {
                    self.matrix[i][j] = Mine::HexCell(true);
                }
            }
        }
    }

    fn pick(a: i32, b: i32) -> i32 {
        let between = Range::new(a, b);
        let mut rng = rand::thread_rng();
        between.ind_sample(&mut rng)
    }

    fn radio(&self, c: &Context) -> f64{
        let rect = c.viewport.unwrap().rect;
        let width = rect[2];
        let height = rect[3];
        let hex_width = width as f64 / ((3.0*self.matrix[0].len() as f64)-1.0);
        let hex_height = height as f64 / (1.0+ self.matrix.len() as f64);
        let width = hex_height.min(hex_width);

        (width) as f64
    }

    fn intersect(&self,poly: Vec<[f64;2]>, pos: &[f64]) -> bool{
                            /*
        https://es.wikipedia.org/wiki/Regla_par-impar
                    num = len(poly)
        j = num - 1
        c = False
        for i in range(num):
                if  ((poly[i][1] > y) != (poly[j][1] > y)) and \
                        (x < (poly[j][0] - poly[i][0]) * (y - poly[i][1]) / (poly[j][1] - poly[i][1]) + poly[i][0]):
                    c = not c
                j = i
        return c
        */
        let x = pos[0];
        let y = pos[1];
        let mut c = false;
        let num = poly.len();
        let mut j = num -1;
        for i in 0..num{
                if  ((poly[i][1] > y) != (poly[j][1] > y)) && (x < (poly[j][0] - poly[i][0]) * (y - poly[i][1]) / (poly[j][1] - poly[i][1]) + poly[i][0]){
                    c = !c
                }
                j = i
        }
        c
    }

    pub fn flag(&mut self, pos: [usize;2]){
        let x = pos[0];
        let y = pos[1];
        self.matrix[x][y] = match &self.matrix[x][y] {
            &Mine::Flag(w) => Mine::HexCell(w),
            &Mine::HexCell(w) => Mine::Flag(w),
            &Mine::Reveal(w) => Mine::Reveal(w),
            &Mine::OutOfTable => Mine::OutOfTable
        };
    }

    pub fn victory(&mut self) -> bool{
        for i in 0..self.matrix.len(){
            for j in 0..self.matrix[i].len(){
                match &self.matrix[i][j]{
                    &Mine::HexCell(false) =>{
                        return false;   
                    },
                    _ => ()
                };
            }
        }
        true
    }

    pub fn reveal(&mut self, pos: [usize;2]) {
        let x = pos[0];
        let mut y = pos[1];
        if std::mem::discriminant(&self.matrix[x][y]) == std::mem::discriminant(&Mine::Reveal(5)){
            return;
        }
        if std::mem::discriminant(&self.matrix[x][y]) == std::mem::discriminant(&Mine::Flag(true)){
            return;
        }
        let mines = self.get_mines_around(pos);
        self.matrix[x][y] = Mine::Reveal(mines);
        if mines == 0 {
            // RECURSIVAMENTE HACER REVEAL EN LAS CASILLAS DE ALREDEDOR
            // COMPROBAR QUE NO ESTAN YA REVELADOS
            // MEJORAR ESTO PARA NO REPETIR CÓDIGO
            if x > 1 {
                self.reveal([x-2,y]); // ARRIBA
            }
            if x < self.matrix.len() -2 {
                self.reveal([x+2,y]); // ABAJO
            }

            y += x % 2;
            // FILA SUPERIOR
            if x > 0 && y > 0{
                self.reveal([x-1,y-1]); // IZQUIERDA
            }
            if x > 0 && y < self.matrix[x-1].len() {
                self.reveal([x-1,y]); // DERECHA
            }
            // FILA INFERIOR
            if x < self.matrix.len() -1  && y > 0 {
                self.reveal([x+1,y-1]); // IZQUIERDA
            }
            if x < self.matrix.len() -1 && y < self.matrix[x+1].len(){
                self.reveal([x+1,y]); // DERECHA
            }
        }
    }

    pub fn is_mine(&self,pos: [usize;2]) -> bool{
        let x = pos[0];
        let y = pos[1];
        self.matrix[x][y].is_mine()
    }

    pub fn get_mines_around(&self,pos: [usize;2]) -> i32{
        let mut mines = 0;
        let x = pos[0];
        let mut y = pos[1];

        if x > 1 {
            mines += self.matrix[x-2][y].mine(); // ARRIBA
        }
        if x < self.matrix.len() -2 {
            mines += self.matrix[x+2][y].mine(); // ABAJO
        }

        y += x % 2;
        // FILA SUPERIOR
        if x > 0 && y > 0{
            mines += self.matrix[x-1][y-1].mine(); // IZQUIERDA
        }
        if x > 0 && y < self.matrix[x-1].len() {
            mines += self.matrix[x-1][y].mine(); // DERECHA
        }
        // FILA INFERIOR
        if x < self.matrix.len() -1  && y > 0 {
            mines += self.matrix[x+1][y-1].mine(); // IZQUIERDA
        }
        if x < self.matrix.len() -1 && y < self.matrix[x+1].len(){
            mines += self.matrix[x+1][y].mine(); // DERECHA
        }
        mines
    }

    pub fn select(&self,pos: &[f64]) -> Option<[usize;2]>{
        let radio = self.radio;
        // devolver la posicion en la matriz de la mina
        let angle: f64 = f64::consts::FRAC_PI_6;

        for i in 0..self.matrix.len(){
            for j in 0..self.matrix[i].len(){
                if self.matrix[i][j] != Mine::OutOfTable{
                    let mut poligono = Vec::new();
                    poligono.push([angle.sin()*radio,0.0]);
                    poligono.push([angle.sin()*radio+radio,0.0]);
                    poligono.push([radio*2.0,angle.cos()*radio]);
                    poligono.push([angle.sin()*radio+radio,angle.cos()*radio*2.0]);
                    poligono.push([angle.sin()*radio,angle.cos()*radio*2.0]);
                    poligono.push([0.0,angle.cos()*radio]);

                    poligono = poligono.into_iter().map(|mut x|{
                        if i % 2 == 1 {
                            x[0] += radio*1.5;
                        }
                        x[0]+=radio*3.0*(j as f64);
                        x[1]+=angle.cos()*radio*(i as f64);
                        x
                    }).collect();

                    if self.intersect(poligono,pos){
                        return Some([i,j]);
                    }

                }
            }
        }
        None
    }

    pub fn draw(&mut self,hex: &G2dTexture,glyphs: &mut Glyphs,c: &Context, g: &mut G2d) 
    {
        let radio: f64 = self.radio(&c);
        self.radio = radio;
        let angle: f64 = f64::consts::FRAC_PI_6;

        for i in 0..self.matrix.len(){
            for j in 0..self.matrix[i].len(){
                if self.matrix[i][j] != Mine::OutOfTable{
                    let mut poligono = Vec::new();
                    poligono.push([angle.sin()*radio,0.0]);
                    poligono.push([angle.sin()*radio+radio,0.0]);
                    poligono.push([radio*2.0,angle.cos()*radio]);
                    poligono.push([angle.sin()*radio+radio,angle.cos()*radio*2.0]);
                    poligono.push([angle.sin()*radio,angle.cos()*radio*2.0]);
                    poligono.push([0.0,angle.cos()*radio]);

                    poligono = poligono.into_iter().map(|mut x|{
                        if i % 2 == 1 {
                            x[0] += radio*1.5;
                        }
                        x[0]+=radio*3.0*(j as f64);
                        x[1]+=angle.cos()*radio*(i as f64);
                        x
                    }).collect();

                    match self.matrix[i][j]{
                        Mine::Reveal(n) => {
                            polygon([0.0,1.0,0.0,1.0],poligono.as_slice(),c.transform,g);
                            if n > 0 {
                                let mut x = radio;
                                let mut y = radio;
                                if i % 2 == 1 {
                                    x += radio*1.5;
                                }
                                x+=radio*3.0*(j as f64);
                                y+=angle.cos()*radio*(i as f64);

                                text([0.0,0.0,1.0,1.0],20,format!("{}",n).as_str(),glyphs,c.transform.trans(x-5.0,y),g).unwrap();
                            }
                        },
                        Mine::Flag(_) => {
                            polygon([1.0,0.0,0.0,1.0],poligono.as_slice(),c.transform,g);
                            let mut x = radio;
                            let mut y = radio;
                            if i % 2 == 1 {
                                x += radio*1.5;
                            }
                            x+=radio*3.0*(j as f64);
                            y+=angle.cos()*radio*(i as f64);

                            text([0.0,0.0,1.0,1.0],16,"F",glyphs,c.transform.trans(x-5.0,y),g).unwrap();
                        },
                        Mine::HexCell(_) => polygon([1.0,0.0,0.0,1.0],poligono.as_slice(),c.transform,g),
                        _ => (),
                    };
                    
                }
            }
        }
    }
}