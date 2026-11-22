slint::slint!
{
    #[edition = "1.19"]
    
    import { CheckBox, VerticalBox } from "std-widgets.slint";
        
    export component Principale inherits Window
    {
        width: 100px;
        height: 200px;
        
        callback check1_coche(bool);
        callback check2_coche(bool);
        callback check3_coche(bool);
        
        VerticalBox
        {
            padding-bottom:10px;
            alignment: center;
            
            CheckBox
            {
                checked: false;
                text: "Case 1";
                toggled => 
                {
                    check1_coche( self.checked );
                }
            }

            CheckBox
            {
                checked: false;
                text: "Case 2";
                toggled => 
                {
                    check2_coche( self.checked );
                }
            }

            CheckBox
            {
                checked: false;
                text: "Case 3";
                toggled => 
                {
                    check3_coche( self.checked );
                }
            }

        }
    }
}

fn main()
{
    let fenetre = Principale::new().unwrap();

    fenetre.on_check1_coche ( | coche | { println!( "Case 1 cochée {coche}" ) } );
    fenetre.on_check2_coche ( | coche | { println!( "Case 2 cochée {coche}" ) } );
    fenetre.on_check3_coche ( | coche | { println!( "Case 3 cochée {coche}" ) } );
    
    fenetre.run().unwrap();
}
