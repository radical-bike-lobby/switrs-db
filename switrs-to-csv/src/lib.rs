use derive_builder::Builder;
use rusqlite::Row;

/// Based off the schema from the SWITRS collisions table.
///
/// This we do not "care" about all fields, so not all will be represented.
///
/// See field definitions in the [RawData Template](https://iswitrs.chp.ca.gov/Reports/jsp/samples/RawData_template.pdf)
///
/// ```text
/// CREATE TABLE IF NOT EXISTS "collisions" (
///   [CASE_ID] INTEGER PRIMARY KEY,
///   [COLLISION_DATE] TEXT,
///   [COLLISION_TIME] INTEGER,
///   [OFFICER_ID] TEXT,
///   [REPORTING_DISTRICT] TEXT,
///   [DAY_OF_WEEK] INTEGER REFERENCES [DAY_OF_WEEK]([id]),
///   [CNTY_CITY_LOC] INTEGER,
///   [BEAT_NUMBER] TEXT,
///   [PRIMARY_RD] TEXT,
///   [SECONDARY_RD] TEXT,
///   [DISTANCE] FLOAT,
///   [DIRECTION] TEXT,
///   [INTERSECTION] TEXT,
///   [WEATHER_1] TEXT REFERENCES [WEATHER_1]([key]),
///   [WEATHER_2] TEXT REFERENCES [WEATHER_2]([key]),
///   [STATE_HWY_IND] TEXT,
///   [CALTRANS_COUNTY] TEXT,
///   [CALTRANS_DISTRICT] INTEGER,
///   [STATE_ROUTE] INTEGER,
///   [POSTMILE] FLOAT,
///   [LOCATION_TYPE] TEXT REFERENCES [LOCATION_TYPE]([key]),
///   [RAMP_INTERSECTION] TEXT REFERENCES [RAMP_INTERSECTION]([key]),
///   [SIDE_OF_HWY] TEXT REFERENCES [SIDE_OF_HWY]([key]),
///   [TOW_AWAY] TEXT,
///   [COLLISION_SEVERITY] INTEGER REFERENCES [COLLISION_SEVERITY]([id]),
///   [NUMBER_KILLED] INTEGER,
///   [NUMBER_INJURED] INTEGER,
///   [PARTY_COUNT] INTEGER,
///   [PRIMARY_COLL_FACTOR] TEXT REFERENCES [PRIMARY_COLL_FACTOR]([key]),
///   [PCF_VIOL_CATEGORY] TEXT REFERENCES [PCF_VIOL_CATEGORY]([key]),
///   [PCF_VIOLATION] INTEGER,
///   [PCF_VIOL_SUBSECTION] TEXT,
///   [HIT_AND_RUN] TEXT,
///   [TYPE_OF_COLLISION] TEXT REFERENCES [TYPE_OF_COLLISION]([key]),
///   [MVIW] TEXT REFERENCES [MVIW]([key]),
///   [PED_ACTION] TEXT REFERENCES [PED_ACTION]([key]),
///   [ROAD_SURFACE] TEXT REFERENCES [ROAD_SURFACE]([key]),
///   [ROAD_COND_1] TEXT REFERENCES [ROAD_COND_1]([key]),
///   [ROAD_COND_2] TEXT REFERENCES [ROAD_COND_2]([key]),
///   [LIGHTING] TEXT REFERENCES [LIGHTING]([key]),
///   [CONTROL_DEVICE] TEXT REFERENCES [CONTROL_DEVICE]([key]),
///   [PEDESTRIAN_ACCIDENT] TEXT,
///   [BICYCLE_ACCIDENT] TEXT,
///   [MOTORCYCLE_ACCIDENT] TEXT,
///   [TRUCK_ACCIDENT] TEXT,
///   [NOT_PRIVATE_PROPERTY] TEXT,
///   [ALCOHOL_INVOLVED] TEXT,
///   [STWD_VEHTYPE_AT_FAULT] TEXT REFERENCES [STWD_VEHTYPE_AT_FAULT]([key]),
///   [CHP_VEHTYPE_AT_FAULT] TEXT REFERENCES [CHP_VEHTYPE_AT_FAULT]([key]),
///   [COUNT_SEVERE_INJ] INTEGER,
///   [COUNT_VISIBLE_INJ] INTEGER,
///   [COUNT_COMPLAINT_PAIN] INTEGER,
///   [COUNT_PED_KILLED] INTEGER,
///   [COUNT_PED_INJURED] INTEGER,
///   [COUNT_BICYCLIST_KILLED] INTEGER,
///   [COUNT_BICYCLIST_INJURED] INTEGER,
///   [COUNT_MC_KILLED] INTEGER,
///   [COUNT_MC_INJURED] INTEGER,
///   [PRIMARY_RAMP] TEXT REFERENCES [PRIMARY_RAMP]([key]),
///   [SECONDARY_RAMP] TEXT REFERENCES [SECONDARY_RAMP]([key]),
///   [LATITUDE] FLOAT,
///   [LONGITUDE] FLOAT,
///   [ADDRESS] TEXT,
///   [SEVERITY_INDEX] TEXT
/// );
/// ```
#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct Collision {
    ///   [CASE_ID] INTEGER PRIMARY KEY,
    /// the unique identifier of the collision report (barcode beginning 2002; 19 digit code prior to 2002)
    pub case_id: usize,
    ///   [COLLISION_DATE] TEXT,
    /// the date when the collision occurred (YYYYMMDD)
    pub collision_date: Option<time::Date>,
    ///   [COLLISION_TIME] INTEGER,
    /// the time when the collision occurred (24 hour time)
    pub collision_time: Option<time::Time>,
    ///   [OFFICER_ID] TEXT,
    pub officer_id: String,
    ///   [REPORTING_DISTRICT] TEXT,
    /// Varchar2(5)
    pub reporting_district: String,
    ///   [DAY_OF_WEEK] INTEGER REFERENCES [DAY_OF_WEEK]([id]),
    /// the code for the day of the week when the collision occurred
    pub day_of_week: Option<time::Weekday>,
    ///   [CNTY_CITY_LOC] INTEGER,
    /// the location code of where the collision occurred
    pub cnty_city_loc: usize,
    ///   [PRIMARY_RD] TEXT,
    pub primary_rd: String,
    ///   [SECONDARY_RD] TEXT,
    pub secondary_rd: String,
    ///   [DISTANCE] FLOAT,
    /// distance converted to feet
    pub distance: f32,
    ///   [DIRECTION] TEXT,
    /// N - North, E - East, S - South, W - West, - or blank - Not Stated, in Intersection
    pub direction: String,
    ///   [INTERSECTION] TEXT,
    /// Y - Intersection, N - Not Intersection, Blank - Not stated
    pub intersection: String,
    ///   [WEATHER_1] TEXT REFERENCES [WEATHER_1]([key]),
    /// A - Clear, B - Cloudy, C - Raining, D - Snowing, E - Fog, F - Other, G - Wind, - - Not Stated
    pub weather_1: String,
    ///   [WEATHER_2] TEXT REFERENCES [WEATHER_2]([key]),
    /// the weather condition at the time of the collision, if a second description is necessary
    pub weather_2: String,
    ///   [STATE_HWY_IND] TEXT,
    /// Y - State Highway, N - Not State Highway, Blank - Not stated
    pub state_hwy_ind: String,
    ///   [CALTRANS_COUNTY] TEXT,
    /// Includes blanks and nulls
    pub caltrans_county: String,
    ///   [CALTRANS_DISTRICT] INTEGER,
    pub caltrans_district: usize,
    ///   [STATE_ROUTE] INTEGER,
    /// 0 = Not State Highway
    pub state_route: usize,
    ///   [POSTMILE] FLOAT,
    pub postmile: f32,
    ///   [LOCATION_TYPE] TEXT REFERENCES [LOCATION_TYPE]([key]),
    /// H - Highway, I - Intersection, R - Ramp (or Collector), - or blank - Not State Highway
    pub location_type: String,
    ///   [RAMP_INTERSECTION] TEXT REFERENCES [RAMP_INTERSECTION]([key]),
    /// 1 - Ramp Exit, Last 50 Feet, 2 - Mid-Ramp, 3 - Ramp Entry, First 50 Feet, 4 - Not State Highway, Ramp-related, Within
    /// 100 Feet, 5 - Intersection, 6 - Not State Highway (Intersection-related Within 250 Feet), 7 - Highway
    pub ramp_intersection: String,
    ///   [SIDE_OF_HWY] TEXT REFERENCES [SIDE_OF_HWY]([key]),
    /// Code provided by Caltrans Coders; applies to divided highway, based on nominal direction of
    ///   route; for single vehicle is same as nominal
    ///   direction of travel, overruled by impact with
    ///   second vehicle after crossing median
    /// N - Northbound, S - Southbound, E - Eastbound, W - Westbound, Blank - Not stated/not state highway
    pub side_of_hwy: String,
    ///   [TOW_AWAY] TEXT,
    /// Y - Yes, N - No
    pub tow_away: String,
    ///   [COLLISION_SEVERITY] INTEGER REFERENCES [COLLISION_SEVERITY]([id]),
    /// the injury level severity of the collision (highest level of injury in collision)
    /// 1 - Fatal, 2 - Injury (Severe), 3 - Injury (Other Visible), 4 - Injury (Complaint of Pain), 0 - PDO
    pub collision_severity: String,
    ///   [NUMBER_KILLED] INTEGER,
    /// counts victims in the collision with degree of injury of 1
    pub number_killed: usize,
    ///   [NUMBER_INJURED] INTEGER,
    /// counts victims in the collision with degree of injury of 2, 3, or 4
    pub number_injured: usize,
    ///   [PARTY_COUNT] INTEGER,
    /// counts total parties in the collision
    pub party_count: usize,
    ///   [PRIMARY_COLL_FACTOR] TEXT REFERENCES [PRIMARY_COLL_FACTOR]([key]),
    /// A - (Vehicle) Code Violation, B - Other Improper Driving, C - Other Than Driver, D - Unknown, E - Fell Asleep, - - Not Stated
    pub primary_coll_factor: String,
    ///   [PCF_VIOL_CATEGORY] TEXT REFERENCES [PCF_VIOL_CATEGORY]([key]),
    /// B - Business and Professions, C - Vehicle, H - City Health and Safety, I - City Ordinance, O - County Ordinance, P - Penal, S - Streets and Highways, W - Welfare and Institutions, - - Not Stated
    pub pcf_viol_category: String,
    ///   [PCF_VIOLATION] INTEGER,
    /// 01 - Driving or Bicycling Under the Influence of Alcohol or Drug, 02 - Impeding Traffic, 03 - Unsafe Speed, 04 - Following Too Closely, 05 - Wrong Side of Road, 06 - Improper Passing, 07 - Unsafe Lane Change, 08 - Improper Turning, 09 - Automobile Right of Way, 10 - Pedestrian Right of Way, 11 - Pedestrian Violation, 12 - Traffic Signals and Signs, 13 - Hazardous Parking, 14 - Lights, 15 - Brakes, 16 - Other Equipment, 17 - Other Hazardous Violation, 18 - Other Than Driver (or Pedestrian), 19 -, 20 -, 21 - Unsafe Starting or Backing, 22 - Other Improper Driving, 23 - Pedestrian or "Other" Under the Influence of Alcohol or Drug, 24 - Fell Asleep, 00 - Unknown, - - Not Stated
    pub pcf_violation: usize,
    ///   [PCF_VIOL_SUBSECTION] TEXT,
    pub pcf_viol_subsection: String,
    ///   [HIT_AND_RUN] TEXT,
    /// F - Felony, M - Misdemeanor, N - Not Hit and Run
    pub hit_and_run: String,
    ///   [TYPE_OF_COLLISION] TEXT REFERENCES [TYPE_OF_COLLISION]([key]),
    /// A - Head-On, B - Sideswipe, C - Rear End, D - Broadside, E - Hit Object, F - Overturned, G - Vehicle/Pedestrian, H - Other, - - Not Stated
    pub type_of_collision: String,
    ///   [MVIW] TEXT REFERENCES [MVIW]([key]),
    /// Motor Vehicle Involved With
    /// A - Non-Collision, B - Pedestrian, C - Other Motor Vehicle, D - Motor Vehicle on Other Roadway, E - Parked Motor Vehicle, F - Train, G - Bicycle, H - Animal, I - Fixed Object, J - Other Object, - - Not Stated
    pub mviw: String,
    ///   [PED_ACTION] TEXT REFERENCES [PED_ACTION]([key]),
    /// A - No Pedestrian Involved, B - Crossing in Crosswalk at Intersection, C - Crossing in Crosswalk Not at Intersection, D - Crossing Not in Crosswalk, E - In Road, Including Shoulder, F - Not in Road, G - Approaching/Leaving School Bus, - - Not Stated
    pub ped_action: String,
    ///   [ROAD_SURFACE] TEXT REFERENCES [ROAD_SURFACE]([key]),
    /// A - Dry, B - Wet, C - Snowy or Icy, D - Slippery (Muddy, Oily, etc.), - - Not Stated
    pub road_surface: String,
    ///   [ROAD_COND_1] TEXT REFERENCES [ROAD_COND_1]([key]),
    /// A - Holes, Deep Ruts, B - Loose Material on Roadway, C - Obstruction on Roadway, D - Construction or Repair Zone, E - Reduced Roadway Width, F - Flooded, G - Other, H - No Unusual Condition, - - Not Stated
    pub road_cond_1: String,
    ///   [ROAD_COND_2] TEXT REFERENCES [ROAD_COND_2]([key]),
    /// same as 1
    pub road_cond_2: String,
    ///   [LIGHTING] TEXT REFERENCES [LIGHTING]([key]),
    /// A - Daylight, B - Dusk - Dawn, C - Dark - Street Lights, D - Dark - No Street Lights, E - Dark - Street Lights Not Functioning, - - Not Stated
    pub lighting: String,
    ///   [CONTROL_DEVICE] TEXT REFERENCES [CONTROL_DEVICE]([key]),
    /// A - Functioning, B - Not Functioning, C - Obscured, D - None, - - Not Stated
    pub control_device: String,
    ///   [PEDESTRIAN_ACCIDENT] TEXT,
    /// indicates whether the collision involved a pedestrian
    /// Y or blank
    pub pedestrian_accident: String,
    ///   [BICYCLE_ACCIDENT] TEXT,
    /// indicates whether the collision involved a bicycle
    /// Y or blank
    pub bicycle_accident: String,
    ///   [MOTORCYCLE_ACCIDENT] TEXT,
    /// indicates whether the collision involved a motorcycle
    /// Y or blank
    pub motorcycle_accident: String,
    ///   [TRUCK_ACCIDENT] TEXT,
    /// indicates whether the collision involved a big truck
    /// Y or blank
    pub truck_accident: String,
    ///   [NOT_PRIVATE_PROPERTY] TEXT,
    /// indicates whether the collision occurred on private property
    /// Y or blank
    pub not_private_property: String,
    ///   [ALCOHOL_INVOLVED] TEXT,
    /// indicates whether the collision involved a party that had been drinking
    /// Y or blank
    pub alcohol_involved: String,
    ///   [STWD_VEHTYPE_AT_FAULT] TEXT REFERENCES [STWD_VEHTYPE_AT_FAULT]([key]),
    /// indicates the Statewide Vehicle Type of the party who is at fault
    pub stwd_vehtype_at_fault: String,
    ///   [CHP_VEHTYPE_AT_FAULT] TEXT REFERENCES [CHP_VEHTYPE_AT_FAULT]([key]),
    /// indicates the CHP Vehicle Type of the party who is at fault
    pub chp_vehtype_at_fault: String,
    ///   [COUNT_SEVERE_INJ] INTEGER,
    /// counts victims in the collision with degree of injury of 2
    pub count_severe_inj: usize,
    ///   [COUNT_VISIBLE_INJ] INTEGER,
    pub count_visible_inj: usize,
    ///   [COUNT_COMPLAINT_PAIN] INTEGER,
    pub count_complaint_pain: usize,
    ///   [COUNT_PED_KILLED] INTEGER,
    pub count_ped_killed: usize,
    ///   [COUNT_PED_INJURED] INTEGER,
    pub count_ped_injured: usize,
    ///   [COUNT_BICYCLIST_KILLED] INTEGER,
    pub count_bicyclist_killed: usize,
    ///   [COUNT_BICYCLIST_INJURED] INTEGER,
    pub count_bicyclist_injured: usize,
    ///   [COUNT_MC_KILLED] INTEGER,
    pub count_mc_killed: usize,
    ///   [COUNT_MC_INJURED] INTEGER,
    pub count_mc_injured: usize,
    ///   [PRIMARY_RAMP] TEXT REFERENCES [PRIMARY_RAMP]([key]),
    pub primary_ramp: usize,
    ///   [SECONDARY_RAMP] TEXT REFERENCES [SECONDARY_RAMP]([key]),
    pub secondary_ramp: usize,
    ///   [LATITUDE] FLOAT,
    pub latitude: f64,
    ///   [LONGITUDE] FLOAT,
    pub longitude: f64,
    ///   [ADDRESS] TEXT,
    pub address: String,
    ///   [SEVERITY_INDEX] TEXT
    pub severity_index: String,
}

impl<'a> TryFrom<&'a Row<'a>> for Collision {
    type Error = rusqlite::Error;

    fn try_from(row: &'a Row<'a>) -> Result<Self, Self::Error> {
        Ok(Collision {
            case_id: row.get("CASE_ID")?,
            ..Default::default()
        })
    }
}
