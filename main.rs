use std::io;
//All game states
enum GameState {
    Intro,
    SearchCell,
    ActivateOdin,
    EscapeCell,
    MeetFrank,
    JuliusCaesarJoke,
    FrankHasPlan,
    UndergroundTunnel,
    TunnelCaesar,
    ExitingJail,
    FightHeimdall,
    HeimdallAlliance,
    HeimdallSwitches,
    HeimdallPortal,
    FrankReturns,
    HeimdallDefeated,
    JuliusCaesarFinal,
    Victory,
    VictoryTerrain,
    VictoryPortal,
    GameOver,
    OdinForest,
    OdinWeird,
    OdinRiver,
    LokiAftermath,
    LokiWeird,
    LokiStation,
    BifrostAlps,
    BifrostWeird,
    BifrostVillage,
    CaesarAscended,
    AbilitiesFail,
    TheImpossibleMove,
    TrueEnding,
}
//Input function
fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}
//Main Game
fn main() {
    let mut current_state = GameState::Intro; //current state

    loop { //game loop
        match current_state {
            //Introduction
            GameState::Intro => {
                println!("\n========================================");
                println!("       O  D  I  N");
                println!("========================================");
                println!("--- SEPTEMBER 1939: RURAL GERMANY ---");
                println!("You are Johann Joestar, age 14.");
                println!("You have been in this cell for three weeks.");
                println!("A guard named Hans patrols the hallway outside. He has the keys.");
                println!("\nWhat do you do?");
                println!("1. Activate ODIN (Puppet Hans)");
                println!("2. Wait and do nothing");
                println!("3. Search the cell for anything useful");

                match get_input().as_str() {
                    "1" => current_state = GameState::ActivateOdin,
                    "3" => current_state = GameState::SearchCell,
                    _ => current_state = GameState::GameOver,
                }
            }
            //Searching the cell
            GameState::SearchCell => {
                println!("\nYou run your hands along the floor, the walls, the base of the cot.");
                println!("A loose stone near the back wall. You pry it out.");
                println!("Behind it: a bent nail, and a scrap of paper folded twice over.");
                println!("The note is written in faded ink:");
                println!("  'The Romans have eyes in the walls. Do not trust the silence.");
                println!("   The nail is yours. Use it wisely.  — D.B.'");
                println!("\nWhat do you do?");
                println!("1. Forget the nail — use ODIN on Hans");
                println!("2. Pick the lock with the nail and slip out quietly");
                println!("3. Put the stone back. This feels like a trap.");

                match get_input().as_str() {
                    "1" => current_state = GameState::ActivateOdin,
                    "2" => {
                        println!("\nYou work the nail into the lock. Fifteen tense seconds.");
                        println!("Your hands are steadier than they have any right to be.");
                        println!("Click.");
                        println!("The door drifts open an inch. Hans has his back to you.");
                        println!("You step into the corridor without making a sound.");
                        current_state = GameState::MeetFrank;
                    }
                    _ => {
                        println!("\nYou replace the stone and sit back down.");
                        println!("An hour later, Hans opens the cell.");
                        println!("He is not bringing food.");
                        current_state = GameState::GameOver;
                    }
                }
            }
            //controlling the gaurd- Hans
            GameState::ActivateOdin => {
                println!("\nYou close your eyes and whisper: 'Hans.'");
                println!("ODIN's threads shoot out through the bars, invisible as silk.");
                println!("Hans freezes mid-step. His eyes cloud over — grey and empty.");
                println!("He is yours now. You feel his heartbeat like it is your own.");
                println!("He walks to the cell door and unlocks it without a word.");
                println!("\nWhat is your next move?");
                println!("1. Take the keys and walk out");
                println!("2. Wait and see what happens");

                match get_input().as_str() {
                    "1" => current_state = GameState::MeetFrank,
                    _ => current_state = GameState::JuliusCaesarJoke,
                }
            }
            //Julius Ceaser takes over Hans's body
            GameState::JuliusCaesarJoke => {
                println!("\nYou hesitate too long.");
                println!("Hans snaps his own neck 180 degrees. The crack echoes off the stone walls.");
                println!("'How dare you think you know me,' he whispers. Deadly quiet.");
                println!("The guard dissolves. In his place stands a man in a white toga and laurel wreath.");
                println!("Julius Caesar. Here. In rural Germany.");
                println!("He snaps your neck before you can speak.");
                println!("In extreme pain, you look up one last time.");
                println!("'Uror & Skuld,' Caesar says pleasantly.");
                println!("Twin figures rise behind him, weaving threads of time.");
                println!("You are dragged into a future void.");
                current_state = GameState::GameOver;
            }
            //Meeting the JoBro
            GameState::MeetFrank => {
                println!("\nYou step into the hallway.");
                println!("'Nice work,' says a familiar voice from the shadows at the far end.");
                println!("\nFrank?");
                println!("\n.....Frank Zeppeli?");
                println!("Your old friend grins — short blonde hair, same age as you, completely unbothered");
                println!("by the fact that you are both inside a German military prison.");
                println!("You exchange stories in thirty seconds flat.");
                println!("Frank's Stand, Loki, bends the physical properties of non-living objects:");
                println!("size, shape, temperature — whatever the situation requires.");
                println!("\nWhat do you do?");
                println!("1. Leave the building together immediately");
                println!("2. Grab Frank by his short blonde hair and throw him out the window");
                println!("3. Ask if Frank already has a plan beyond 'walk out the front door'");

                match get_input().as_str() {
                    "1" => current_state = GameState::ExitingJail,
                    "3" => current_state = GameState::FrankHasPlan,
                    _ => current_state = GameState::EscapeCell,
                }
            }
            // getting flamed by Frank (literally)
            GameState::EscapeCell => {
                println!("\nFrank's expression does not change.");
                println!("'Interesting choice,' he says.");
                println!("Loki activates. The temperature of your clothes rises 180 degrees Celsius.");
                println!("You explode into flames.");
                println!("Bad choice. :C");
                current_state = GameState::GameOver;
            }
            //Frank Has a Plan
            GameState::FrankHasPlan => {
                println!("\n'Actually,' Frank says, 'I do.'");
                println!("He pulls out a rough sketch drawn on the back of a guard's patrol sheet.");
                println!("'While Loki was scouting ahead, I found a drainage channel.");
                println!("It runs under the outer wall and comes out in a field about two hundred metres north.'");
                println!("He taps the sketch. 'The front gate is the obvious move.");
                println!("Whoever set this prison up expected us to try the front gate.'");
                println!("\nWhat do you do?");
                println!("1. Take the tunnel");
                println!("2. The front door is faster — stick to the original plan");

                match get_input().as_str() {
                    "1" => current_state = GameState::UndergroundTunnel,
                    _ => current_state = GameState::ExitingJail,
                }
            }
            //You guys go underground
            GameState::UndergroundTunnel => {
                println!("\nYou find the drainage grate behind a storeroom on the ground floor.");
                println!("Loki bends the iron bars aside without a sound.");
                println!("You drop in. Frank drops in behind you.");
                println!("It is cold, pitch-black, and smells like three decades of stone water.");
                println!("You crawl.");
                println!("Halfway through, ODIN sends a warning — not in words, just a cold weight");
                println!("behind your eyes that means: something has noticed the tunnel.");
                println!("'Above us,' ODIN confirms. 'Footsteps. Patient. That is not a guard's rhythm.'");
                println!("'Bifrost is walking the outer wall. He has not locked onto you yet.");
                println!("He is scanning above ground.'");
                println!("\nWhat do you do?");
                println!("1. Move slowly. Have ODIN redirect Heimdall's attention.");
                println!("2. Rush through before he can scan below ground.");

                match get_input().as_str() {
                    "1" => {
                        println!("\nODIN reaches upward through ten centimetres of stone.");
                        println!("A flicker at the treeline. A shadow that wasn't there a second ago.");
                        println!("Heimdall's gaze swings away from the tunnel line.");
                        println!("You crawl. Frank crawls. One movement every five seconds.");
                        println!("Then: cold air rushing in. A rusted grate. Wet grass beyond it.");
                        println!("Loki turns the grate to powder.");
                        current_state = GameState::TunnelCaesar;
                    }
                    _ => {
                        println!("\nYour knee scrapes stone. The sound carries.");
                        println!("Bifrost flares. The tunnel itself warps — walls folding, direction inverting.");
                        println!("You crawl out of the grate and find yourself back in your cell.");
                        current_state = GameState::ExitingJail;
                    }
                }
            }
            //You fight ceaser after leaving the tunnel
            GameState::TunnelCaesar => {
                println!("\nYou haul yourself out of the drainage grate into open air.");
                println!("Wet grass. Pine. The outer prison wall ten metres to your left.");
                println!("Frank climbs out beside you. You both stand up.");
                println!("For ten full seconds: nothing. Just the horizon turning pale gold.");
                println!("'We actually—' Frank starts.");
                println!("A slow exhale from the direction of the birch tree line.");
                println!("'The tunnel. I should have sealed it the moment I arrived.'");
                println!("Julius Caesar steps out from between two trees, hands clasped behind his back.");
                println!("He does not look defeated. He looks inconvenienced.");
                println!("'I saw most paths. Not this one. That is... notable.'");
                println!("Uror & Skuld rises behind him — but slower than you expected.");
                println!("Less certain. Caesar needed time to read this future, and you denied it to him.");
                println!("\nWhat do you do?");
                println!("1. Attack — he is off balance for the first time");
                println!("2. Use ODIN while his Stand is still recovering");
                println!("3. Have Frank reshape the terrain under him");

                match get_input().as_str() {
                    "1" => {
                        println!("\nYou lunge forward.");
                        println!("Uror & Skuld snaps back into full form in half a second.");
                        println!("Time stutters. You are fighting a future version of yourself again.");
                        println!("It ends the same way it always does.");
                        println!("You are dead.");
                        current_state = GameState::GameOver;
                    }
                    "2" => current_state = GameState::Victory,
                    _ => current_state = GameState::VictoryTerrain,
                }
            }
            // Bifrosts' Loop
            GameState::ExitingJail => {
                println!("\nYou and Frank move through the prison corridors.");
                println!("Every guard that steps into your path is vaporised by Loki before they can shout.");
                println!("The final door is ahead. You push through it—");
                println!("—and you are back in your cell.");
                println!("Bars. Stone floor. The same three walls.");
                println!("'Frank???' you call out.");
                println!("Silence.");
                println!("A faint glimmer of light in the corner of the room.");
                println!("\nWhat do you do?");
                println!("1. Order the glimmer to reveal itself");
                println!("2. Have ODIN sweep the cell with his all-seeing eye");

                match get_input().as_str() {
                    "2" => current_state = GameState::FightHeimdall,
                    _ => {
                        println!("\nNothing happens.");
                        println!("A second later your body has been sliced cleanly in half.");
                        println!("You are dead.");
                        current_state = GameState::GameOver;
                    }
                }
            }
            //Fighting Heimdall
            GameState::FightHeimdall => {
                println!("\nODIN's all-seeing eye sweeps the cell.");
                println!("There — corner — a figure. Invisible. Patient. Waiting.");
                println!("You can feel the Stand's presence: cold, Norse, ancient.");
                println!("ODIN whispers one word into your skull: 'Bifrost.'");
                println!("A gatekeeper Stand. It has been looping you back here every time you escape.");
                println!("\nWhat do you do?");
                println!("1. Call for Zeppeli");
                println!("2. Speak to the invisible Stand user directly");
                println!("3. Go to sleep");

                match get_input().as_str() {
                    "1" => current_state = GameState::FrankReturns,
                    "2" => current_state = GameState::HeimdallAlliance,
                    _ => {
                        println!("\nYou close your eyes. The last thing you feel is a cold blade.");
                        println!("You are dead.");
                        current_state = GameState::GameOver;
                    }
                }
            }
            //Alliance wihth Heimdall
            GameState::HeimdallAlliance => {
                println!("\n'ODIN — his name.'");
                println!("'Heimdall,' ODIN answers. 'The Watchman of Asgard. He guards the bridge");
                println!("between worlds for gods, not for emperors.'");
                println!("\nYou speak to the empty corner.");
                println!("'Heimdall. I know who you are.'");
                println!("The glimmer shifts.");
                println!("'You stand watch for Asgard. You were made for the rainbow bridge,");
                println!("not for a military prison in rural Germany.");
                println!("Julius Caesar is a man. Roman. Dead nearly two thousand years and yet here.");
                println!("How long have you owed him this debt?'");
                println!("A long silence. The invisible figure is very still.");
                println!("\nWhat do you do?");
                println!("1. Stay quiet. Let him think.");
                println!("2. You are running out of patience — use ODIN on him now.");

                match get_input().as_str() {
                    "1" => current_state = GameState::HeimdallSwitches,
                    _ => {
                        println!("\nODIN's threads launch before Heimdall has finished thinking.");
                        println!("Caught mid-hesitation, his defences are down.");
                        current_state = GameState::HeimdallDefeated;
                    }
                }
            }

            //Alliance continuation
            GameState::HeimdallSwitches => {
                println!("\nHeimdall fully materialises.");
                println!("He is taller than you expected. Norse runes on black armour, glowing faint gold.");
                println!("His Stand, Bifrost, blooms behind him — prismatic, vast.");
                println!("He is quiet for a long time.");
                println!("'The debt is old,' he says at last. His voice is like a door opening.");
                println!("'Caesar pulled me from the void when my Stand had no anchor.");
                println!("I owed him a term of service. A fixed number.'");
                println!("He looks at the far wall. 'You are the ninth return. That was his number.");
                println!("The debt is paid as of tonight.'");
                println!("He turns to face you.");
                println!("'I will not fight you. And I will not send you back again.'");
                println!("He raises one hand. Bifrost blooms across the entire cell wall.");
                println!("'I can open a gate. Anywhere I have stood.'");
                println!("\nWhere do you go?");
                println!("1. Outside the outer wall — anywhere, just out");
                println!("2. Somewhere Caesar cannot follow");

                match get_input().as_str() {
                    "1" | "2" => current_state = GameState::HeimdallPortal,
                    _ => {
                        println!("\nHeimdall waits. The gate begins to dim.");
                        println!("'The offer does not last.'");
                        println!("The gate closes. He is gone. You are alone.");
                        current_state = GameState::GameOver;
                    }
                }
            }
            //you run away
            GameState::HeimdallPortal => {
                println!("\nThe gate opens.");
                println!("Not a rainbow. Just a door-shaped absence in the stone wall, through which you");
                println!("can see a hillside at dawn — green, impossible, real.");
                println!("Frank appears at your shoulder. He looks at the gate. He looks at Heimdall.");
                println!("He looks at you.");
                println!("He shrugs and walks through first.");
                println!("You follow. Heimdall steps through last.");
                println!("The gate closes behind him with a sound like a lock that will never reopen.");
                current_state = GameState::VictoryPortal;
            }
            //frank saves you
            GameState::FrankReturns => {
                println!("\n'FRANK!'");
                println!("Your voice hits the stone walls and dies.");
                println!("A beat of silence.");
                println!("Then — CRACK. The wall to your right splinters inward.");
                println!("Frank Zeppeli steps through the rubble, plaster dust in his blonde hair,");
                println!("completely calm.");
                println!("'Something kept pulling me back to the outside every time I tried to re-enter.'");
                println!("He scans the room. 'Where?'");
                println!("'Corner. ODIN can see the outline.'");
                println!("Frank raises his hand. Loki activates.");
                println!("The temperature in the corner drops sharply. The air warps.");
                println!("A figure materialises — tall, pale, Norse runes carved into black armour.");
                println!("A prismatic gate of light pulses behind him like folded wings.");
                println!("'Heimdall,' ODIN confirms. 'Guardian. Gatekeeper. Loyal to Caesar.'");
                println!("\nWhat do you do?");
                println!("1. Use ODIN to puppet Heimdall");
                println!("2. Have Frank use Loki to destroy the Bifrost gate directly");
                println!("3. Charge at Heimdall");

                match get_input().as_str() {
                    "1" => current_state = GameState::HeimdallDefeated,
                    "2" => {
                        println!("\nFrank drives his hands downward. Loki reaches into the light.");
                        println!("Bifrost is not matter — but it is energy, and energy has temperature.");
                        println!("Frank drops it toward absolute zero.");
                        println!("The gate fractures. Heimdall screams — the first sound he has made.");
                        println!("His Stand shatters like glass, and he crumples with it.");
                        println!("'That,' Frank says, shaking his hands out, 'was unpleasant.'");
                        println!("\nYou both run.");
                        current_state = GameState::JuliusCaesarFinal;
                    }
                    _ => {
                        println!("\nYou sprint forward. Bifrost erupts.");
                        println!("A blade of rainbow light opens across your chest. You are dead.");
                        current_state = GameState::GameOver;
                    }
                }
            }   
            //you defeat heimdall
            GameState::HeimdallDefeated => {
                println!("\n'Heimdall.'");
                println!("The name leaves your mouth like a command.");
                println!("ODIN's threads launch across the room.");
                println!("Heimdall snarls — Bifrost blazes, trying to incinerate the threads mid-flight.");
                println!("'Frank — the gate!'");
                println!("Frank slams both palms down. Loki drops the temperature of the entire room.");
                println!("Bifrost dims. Heimdall staggers. ODIN's threads sink home.");
                println!("Heimdall's eyes go grey. His Stand dissolves into light and then nothing.");
                println!("He crumples to the floor.");
                println!("'Not a bad combo,' Frank says, cracking his knuckles.");
                println!("\nYou both run.");
                current_state = GameState::JuliusCaesarFinal;
            }
            //Julius Ceaser Final Fight
            GameState::JuliusCaesarFinal => {
                println!("\n--- THE EXIT GATE ---");
                println!("You and Frank tear through the last corridor.");
                println!("Loki vaporises two guards without Frank breaking stride.");
                println!("The front gate is right there. Twenty metres. Ten. Five.");
                println!("A slow clap echoes through the stone archway.");
                println!("Julius Caesar steps from the shadows — white toga, laurel wreath, utterly bored.");
                println!("'Two children. You defeated Heimdall.' A pause. 'I am genuinely surprised.'");
                println!("His Stand erupts behind him: Uror & Skuld.");
                println!("Twin figures — one gazing forward into what will be, one backward into what was —");
                println!("stitching threads of time between their fingers like a loom.");
                println!("'Every path you take ends here,' Caesar says. 'I have already seen all of them.'");
                println!("\nWhat do you do?");
                println!("1. Attack Julius Caesar directly");
                println!("2. Use ODIN on Julius Caesar");
                println!("3. Have Frank collapse the archway onto him");
                println!("4. Run through the gate");

                match get_input().as_str() {
                    "1" => {
                        println!("\nYou charge. Uror & Skuld pull a single thread.");
                        println!("Time stutters. You find yourself fighting a version of yourself from three");
                        println!("seconds in the future — every move you make, it has already made.");
                        println!("You are dead.");
                        current_state = GameState::GameOver;
                    }
                    "2" => current_state = GameState::Victory,
                    "3" => {
                        println!("\nFrank turns his palms upward. Loki grips the stone arch.");
                        println!("Caesar sees it. Uror pulls a thread — time rewinds two seconds.");
                        println!("The arch is intact. Frank has not moved yet.");
                        println!("'I have already seen every move your friend makes,' Caesar says.");
                        println!("'Loki is powerful. But I have had twenty centuries to study its limits.'");
                        println!("Skuld pulls a second thread. The corridor inverts.");
                        println!("You are back in your cell.");
                        current_state = GameState::GameOver;
                    }
                    _ => {
                        println!("\nYou bolt for the gate. Caesar raises one finger.");
                        println!("Skuld pulls a thread. The gate warps.");
                        println!("You step through and find yourself back in your cell.");
                        println!("'Did you really think I hadn't seen this?' Caesar says from everywhere.");
                        current_state = GameState::GameOver;
                    }
                }
            }
            //Victory
            GameState::Victory => {
                println!("\nYou stop running.");
                println!("Caesar tilts his head. 'What are you—'");
                println!("'Julius.'");
                println!("You let the name sit in the air between you.");
                println!("The temporal threads tremble.");
                println!("'Caesar.'");
                println!("ODIN launches. Threads of will, thin as spider silk,");
                println!("crossing twenty years of legend and reaching for a living mind.");
                println!("Uror & Skuld screams — a sound like tearing calendar pages.");
                println!("Caesar's eyes cloud over. Grey. Empty.");
                println!("He knew this moment was coming. He just never believed");
                println!("it would come from a fourteen-year-old.");
                println!("Frank doesn't wait. Loki reshapes the iron gate into a ramp in four seconds flat.");
                println!("You run.");
                println!("Behind you, the future void collapses inward on itself.");
                println!("Julius Caesar folds into his own Stand and is gone.");
                current_state = GameState::OdinForest;
            }
            //another victory
            GameState::VictoryTerrain => {
                println!("\n'Frank.'");
                println!("He is already moving. Loki reaches into the earth.");
                println!("Clay, soil, and stone begin to behave like liquid under his hands.");
                println!("The ground under Caesar's feet shifts. Tilts. Softens.");
                println!("Uror & Skuld begins pulling time threads — but the footing gives way");
                println!("faster than the Stand can compensate.");
                println!("Caesar stumbles. For a fraction of a second, Skuld's backward gaze breaks.");
                println!("ODIN launches into the gap.");
                println!("The threads find Caesar's mind before he can recover his footing.");
                println!("His eyes grey out mid-fall.");
                println!("Frank brings the ground back to solid.");
                println!("Caesar stands in the field: upright, rigid, empty.");
                println!("His Stand dissolves.");
                println!("'Tell Skuld to stand down,' you say.");
                println!("The temporal loom goes dark.");
                println!("You hold him there for exactly as long as it takes to walk past him.");
                println!("At the treeline you release ODIN.");
                println!("Behind you, Julius Caesar collapses into the wet grass.");
                current_state = GameState::LokiAftermath;
            }
            //final victory
            GameState::VictoryPortal => {
                println!("\nYou are standing on a hillside.");
                println!("Behind you: a closed gate that is also a closed wall that is also nothing.");
                println!("The Alps rise in three directions. The sky is pink-grey at the horizon.");
                println!("Frank sits down in the grass immediately.");
                println!("Heimdall looks out across the valley for a long moment.");
                println!("'Caesar cannot follow without the gate,' he says. 'And I am the gate.");
                println!("I will not be returning to him.'");
                println!("He does not say anything after that.");
                println!("Neither do you.");
                println!("Frank finds a pine cone and spins it on one finger.");
                println!("'Switzerland,' he says, looking at the mountains. 'Could be worse.'");
                println!("Johann Joestar sits down next to Frank Zeppeli");
                println!("and watches the sun come up over the Alps.");
                current_state = GameState::BifrostAlps;
            }
            //Game Over
            GameState::GameOver => {
                println!("\n--- GAME OVER ---");
                println!("1. Try Again");
                println!("2. Quit");

                match get_input().as_str() {
                    "1" => current_state = GameState::Intro,
                    _ => break,
                }
            }
            //ODIN Path - Forest
            GameState::OdinForest => {
                println!("\n--- SEPTEMBER 1939. RURAL GERMANY. THE SUN IS COMING UP. ---");
                println!("Frank puts his hands behind his head and exhales slowly.");
                println!("'Not bad for a pair of fourteen-year-olds.'");
                println!("Johann Joestar smiles for the first time in three weeks.");
                println!("\nYou and Frank sprint into the treeline.");
                println!("Through the pines — dogs. A sweep team, mobilised faster than you expected.");
                println!("Frank slows. 'There is a farmhouse a hundred metres east.'");
                println!("A lamp in the window.");
                println!("\nWhat do you do?");
                println!("1. Head for the farmhouse");
                println!("2. Keep running deeper into the forest");

                match get_input().as_str() {
                    "1" => current_state = GameState::OdinWeird,
                    _ => current_state = GameState::GameOver,
                }
            }
            //ODIN Path - Bizarre Encounter
            GameState::OdinWeird => {
                println!("\nYou are in the barn. Hay, livestock smell, grey light through the slats.");
                println!("The farmer's wife enters to feed the chickens.");
                println!("She sees you both. Her expression does not change.");
                println!("She says, in perfect formal Latin, without any apparent surprise:");
                println!("  'Non effugies a Iulio.'");
                println!("  (You will not escape Julius.)");
                println!("Then she goes back to feeding the chickens.");
                println!("Frank leans close. 'Did she just—'");
                println!("'Don't.'");
                println!("'Johann, she spoke—'");
                println!("'I know. Don't.'");
                println!("In the corner of the barn, a goat is watching you with the patient expression");
                println!("of something that has known about you for a long time.");
                println!("\nWhat do you do?");
                println!("1. Leave now — move toward the river");
                println!("2. Ask the farmer about his wife");

                match get_input().as_str() {
                    "1" => current_state = GameState::OdinRiver,
                    "2" => {
                        println!("\nThe farmer looks confused. 'My wife died in 1931.'");
                        println!("You and Frank do not ask follow-up questions.");
                        current_state = GameState::OdinRiver;
                    }
                    _ => current_state = GameState::OdinRiver,
                }
            }
            //ODIN Path - River Crossing
            GameState::OdinRiver => {
                println!("\nNightfall. You have been in that barn for eleven hours.");
                println!("The river is thirty metres wide. Czech territory on the far bank.");
                println!("There is one border guard, smoking, facing away.");
                println!("Frank: 'I can slow the current by reshaping the riverbed upstream.'");
                println!("'But the ground shift will be audible.'");
                println!("\nWhat do you do?");
                println!("1. ODIN — puppet the guard, have him face the other direction");
                println!("2. Let Frank reshape the bed, swim fast before he turns around");

                match get_input().as_str() {
                    "1" => {
                        println!("\nODIN's threads drift upward through the darkness.");
                        println!("The guard turns slowly. He faces the river now. Away from you.");
                        println!("You wade in. Frank wades in behind you. The water is ice.");
                        println!("Your feet touch the far bank. Czech mud.");
                        current_state = GameState::CaesarAscended;
                    }
                    _ => {
                        println!("\nFrank drops both palms flat. Loki reaches down.");
                        println!("The riverbed shifts. The current slows to a crawl for exactly four seconds.");
                        println!("You both thrash across. The far bank mud. Czech territory.");
                        current_state = GameState::CaesarAscended;
                    }
                }
            }
            //LOKI Path - Aftermath
            GameState::LokiAftermath => {
                println!("\nCaesar is face-down in the wet grass. His Stand is gone.");
                println!("Frank wipes his hands on his trousers.");
                println!("'He will wake up in an hour. Maybe two.'");
                println!("Somewhere behind you — an engine. Military vehicle on the perimeter road.");
                println!("The temporal loom dissolved when Caesar fell.");
                println!("Uror and Skuld are gone.");
                println!("Or they were. Something catches your eye: a faint thread in the air above the grass,");
                println!("spinning slowly with no one attached to it.");
                println!("Then it winks out.");
                println!("\nWhat do you do?");
                println!("1. Move now — head east toward the train lines");
                println!("2. Stay another minute — watch where that thread went");

                match get_input().as_str() {
                    "1" => current_state = GameState::LokiWeird,
                    _ => {
                        println!("\nYou wait. The thread does not reappear.");
                        println!("But you saw it. You both saw it.");
                        current_state = GameState::LokiWeird;
                    }
                }
            }
            //LOKI Path - Bizarre Encounter
            GameState::LokiWeird => {
                println!("\nA rural station. One platform, one lamp, a timetable behind glass.");
                println!("The westbound train is in twenty minutes.");
                println!("A child is sitting on the platform bench, playing with wooden soldier figurines.");
                println!("One of them is Julius Caesar.");
                println!("Not a general figurine. Not a Roman toy soldier.");
                println!("Julius Caesar specifically — same face, same coat, same posture you saw in the field.");
                println!("The child is not looking at it.");
                println!("The figurine is facing you.");
                println!("Frank sees it too. He does not say anything. He is very still.");
                println!("The train arrives.");
                println!("When it pulls out six minutes later, you are on it.");
                println!("The bench is empty.");
                println!("\nWhat do you do?");
                println!("1. You are on the train. Keep moving.");
                println!("2. You took the figurine.");

                match get_input().as_str() {
                    "1" => current_state = GameState::LokiStation,
                    "2" => {
                        println!("\nIt is cold wood. Just cold wood. But you keep it.");
                        current_state = GameState::LokiStation;
                    }
                    _ => current_state = GameState::LokiStation,
                }
            }
            //LOKI Path - Border Checkpoint
            GameState::LokiStation => {
                println!("\nFrench border. Late afternoon. A checkpoint officer boards at Saarbrücken.");
                println!("Young. Bored. Routine — until he reaches your row and stops.");
                println!("'Papers.'");
                println!("Frank has not moved. You have not moved.");
                println!("\nWhat do you do?");
                println!("1. ODIN — puppet him, send him on to the next car");
                println!("2. Tell him you are students returning from a school trip, lost your papers");

                match get_input().as_str() {
                    "1" => {
                        println!("\nODIN launches. The officer's eyes grey. He nods and moves on.");
                        println!("You release him three cars down. He blinks and keeps walking.");
                        current_state = GameState::CaesarAscended;
                    }
                    _ => {
                        println!("\nHe does not believe it. He calls ahead. Your names are put on a list.");
                        println!("The train is stopped at the next station. You are removed.");
                        current_state = GameState::GameOver;
                    }
                }
            }
            //BIFROST Path - Alps
            GameState::BifrostAlps => {
                println!("\nThe gate closes with no sound.");
                println!("Heimdall stands at the edge of the hillside, looking back at where it was.");
                println!("'Caesar will attempt to re-open it,' he says. 'Not today. But eventually.'");
                println!("'Bifrost is not mine alone. He knows the resonance frequency.'");
                println!("'We have a window. Perhaps three days.'");
                println!("Frank: 'Before he can reopen it from the German side?'");
                println!("Heimdall: 'Before he completes what he started.'");
                println!("Neither of you asks what that means.");
                println!("You should have asked what that means.");
                println!("\nWhat do you do?");
                println!("1. Ask what he means by 'what he started'");
                println!("2. Focus on moving — find shelter before nightfall");

                match get_input().as_str() {
                    "1" => {
                        println!("\nHeimdall says: 'When your Stand collapsed his, you did not destroy him.'");
                        println!("'You accelerated him.'");
                        current_state = GameState::BifrostWeird;
                    }
                    _ => current_state = GameState::BifrostWeird,
                }
            }
            //BIFROST Path - Bizarre Encounter
            GameState::BifrostWeird => {
                println!("\nYou find the village at the base of the valley just before dark.");
                println!("A postman on a bicycle stops in front of you.");
                println!("He checks a slip of paper.");
                println!("He hands you an envelope, addressed to:");
                println!("    THE PUPPET BOY");
                println!("    HILLSIDE");
                println!("    SWITZERLAND");
                println!("The handwriting matches the note from your cell wall.");
                println!("Inside: one page. A drawing of Julius Caesar standing on a mountain,");
                println!("arms spread, surrounded by light. Beneath it, two words:");
                println!("\n    HE ASCENDED.");
                println!("                — D.B.");
                println!("\nHeimdall looks at the drawing for a long time.");
                println!("Frank: 'Who is D.B.?'");
                println!("No one answers.");
                println!("\nWhat do you do?");
                println!("1. Show Heimdall the letter");
                println!("2. Pocket it and say nothing");

                match get_input().as_str() {
                    "1" => {
                        println!("\nHeimdall stares at it. 'I have seen that posture before.'");
                        println!("'In someone who forgot what they were.'");
                        current_state = GameState::BifrostVillage;
                    }
                    _ => current_state = GameState::BifrostVillage,
                }
            }
            //BIFROST Path - Village
            GameState::BifrostVillage => {
                println!("\nAn old woman outside a bakery watches three strange young men walk down her road.");
                println!("One of them is armoured in Norse runes.");
                println!("None of them have papers.");
                println!("She goes inside and comes back with bread.");
                println!("You eat it standing in the road.");
                println!("Heimdall looks north.");
                println!("'Something is coming through the mountains,' he says.");
                println!("'Not through my gate. Through the other kind.'");
                println!("\nWhat do you do?");
                println!("1. Ask what the other kind is");
                println!("2. You already know");

                match get_input().as_str() {
                    "1" => {
                        println!("\nHeimdall says: 'Time has gates too.'");
                        current_state = GameState::CaesarAscended;
                    }
                    _ => current_state = GameState::CaesarAscended,
                }
            }
            //Caesar Ascended
            GameState::CaesarAscended => {
                println!("\nJulius Caesar steps out of the sun.");
                println!("Not from shade. From the sun itself.");
                println!("His skin is the colour of cold marble. His eyes do not reflect light — they absorb it.");
                println!("Uror and Skuld are not beside him. They are inside him.");
                println!("The air around his body bends slightly, the way hot pavement bends it —");
                println!("but it is September, and it is cold.");
                println!("\nHe says:");
                println!("  'I have been waiting in thirty-seven of the thirty-nine versions of this moment.'");
                println!("  'In the other two, you ran.'");
                println!("\nHe tilts his head.");
                println!("  'I was curious what you would do in the remaining ones.'");
                println!("\nWhat do you do?");
                println!("1. ODIN — launch the threads");
                println!("2. Frank — terrain, now");
                println!("3. Run");

                match get_input().as_str() {
                    "1" | "2" | "3" => current_state = GameState::AbilitiesFail,
                    _ => current_state = GameState::AbilitiesFail,
                }
            }
            //Abilities Fail
            GameState::AbilitiesFail => {
                println!("\nODIN fires.");
                println!("The threads cross the air and find — nothing. Not resistance. Nothing.");
                println!("Like casting a net into fog that has no far side.");
                println!("Caesar: 'Your threads require a node. One mind, one location.'");
                println!("'Mine is distributed across seventeen simultaneous moments.'");
                println!("'Which one would you like to puppet?'");
                println!("\nFrank does not wait. Loki pushes into the ground beneath Caesar's feet—");
                println!("Caesar is already two seconds earlier. He repositioned before Frank's hands moved.");
                println!("'The terrain trick. I saw it in Uror's left index finger, four hours ago.'");
                println!("He almost sounds bored.");
                println!("'Charming. Thoroughly ineffective.'");
                println!("\nHe raises one hand. Not threateningly. The gesture of someone citing a fact.");
                println!("'Shall we discuss the nature of inevitability?'");
                println!("\nWhat do you do?");
                println!("1. Keep fighting (all you have left)");
                println!("2. Stop. Think. There has to be something he cannot foresee.");

                match get_input().as_str() {
                    "1" => current_state = GameState::GameOver,
                    _ => current_state = GameState::TheImpossibleMove,
                }
            }
            //The Impossible Move
            GameState::TheImpossibleMove => {
                println!("\nCaesar sees through Uror (past) and Skuld (future).");
                println!("Every SIGNIFICANT action creates a thread he can trace.");
                println!("But something with no causal weight — something meaningless, something that LEADS nowhere");
                println!("and comes from nowhere — produces no thread at all.");
                println!("\nYou look at Frank. 'What is the most pointless, stupid thing you can do with Loki right now.'");
                println!("Frank: 'What?'");
                println!("'Not a weapon. Not a tactic. The most meaningless thing.'");
                println!("\nCaesar's temporal shimmer hesitates. For the first time.");
                println!("\nFrank looks at the ground. He finds a pine cone. He picks it up.");
                println!("He uses Loki on it.");
                println!("He makes it warm.");
                println!("Just warm.");
                println!("\nCaesar: 'What are you—'");
                println!("\nA warm pine cone has never appeared in any of Uror & Skuld's timelines.");
                println!("In five centuries of operation, it has never once been significant.");
                println!("It cannot be traced forward. It cannot be traced backward. It means nothing.");
                println!("\nThe gap in Caesar's temporal sight lasts 0.4 seconds.");
                println!("\nYou do not use ODIN on Caesar.");
                println!("You use ODIN on the thermal shimmer his body produces —");
                println!("the cold-marble surface that absorbs all light and converts it to nothing.");
                println!("The feedback threads wrap not around a mind but around a temperature signature.");
                println!("\nThen Frank does the rest.");
                println!("Loki does not need to produce sunlight.");
                println!("It only needs to produce the exact thermal signature of direct August sun: 50 degrees Celsius.");
                println!("Focused. At the standing shimmer. For 0.4 seconds.");
                println!("\nJulius Caesar, Ultimate Superior Being, immortal vampire immune to actual sunlight—");
                println!("—reacts to simulated sunlight exactly the same way.");
                println!("\nHe begins to scream. It is not a human sound.");
                println!("\nWhat do you do?");
                println!("1. Hold it — maintain ODIN and Loki together");
                println!("2. Release — it is too much to hold");

                match get_input().as_str() {
                    "1" => current_state = GameState::TrueEnding,
                    _ => current_state = GameState::GameOver,
                }
            }
            //True Ending
            GameState::TrueEnding => {
                println!("\nHe does not die.");
                println!("That was never the plan.");
                println!("A Superior Being cannot be killed by two fourteen-year-olds");
                println!("and a Norse gatekeeper who decided he had enough.");
                println!("\nBut he can be sent somewhere.");
                println!("\nThe temporal loom collapses inward.");
                println!("Not because Caesar wants it to.");
                println!("Because a mind distributed across seventeen simultaneous moments");
                println!("cannot handle all seventeen screaming at once.");
                println!("\nHe retreats to the one place no one can follow:");
                println!("the space between seconds.");
                println!("\nThe shimmer dissipates. The light goes back to normal.");
                println!("Frank is still holding the pine cone.");
                println!("He looks at it.");
                println!("He looks at you.");
                println!("\n'Did we just defeat a god with a warm pine cone.'");
                println!("'Don't.'");
                println!("'Johann—'");
                println!("'I am serious. Don't.'");
                println!("\nHe throws it into the bushes.");
                println!("\nJulius Caesar is not dead. You know that.");
                println!("He is somewhere that is not quite any time,");
                println!("watching seventeen versions of this moment,");
                println!("trying to find the exact moment it went wrong.");
                println!("\nYou decide not to think about that for a while.");
                println!("\n--- SEPTEMBER 1939. THE WAR HAS JUST STARTED. ---");
                println!("You have no papers, no money, and one friend who can make things warm.");
                println!("That will have to be enough.");
                println!("\n═══════════════════════════════════════════════════════");
                println!("          →  To Be Continued...");
                println!("═══════════════════════════════════════════════════════");
                println!("\n1. Play Again");
                println!("2. Quit");

                match get_input().as_str() {
                    "1" => current_state = GameState::Intro,
                    _ => break,
                }
            }
        }
    }
}
