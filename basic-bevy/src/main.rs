use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}

struct PeoplePlugin;
impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, print_names)
            .add_systems(Update, people_with_jobs)
            .add_systems(Update, people_ready_for_hire)
            .add_systems(Update, person_does_job);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn((
        Person {
            name: "Charlie".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((
        Person {
            name: "David".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));
    commands.spawn((
        Person {
            name: "Ellen".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
}

fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name {0}", person.name);
    }
}

fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{0} has a job.", person.name);
    }
}

fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{0} is ready for hire.", person.name);
    }
}

fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "Fire Fighter",
            Job::Lawyer => "Lawyer",
        };

        println!("{0} is a {1}", person.name, job_name)
    }
}

#[derive(Component)]
struct Person {
    pub name: String,
}

#[derive(Component)]
struct Employed {
    pub job: Job,
}

#[derive(Debug)]
enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
