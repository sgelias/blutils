use crate::domain::dtos::{
    blast_builder::Taxon::{self, *},
    blast_result::ValidTaxonomicRanksEnum::{self, *},
};

use clean_base::utils::errors::MappedErrors;

/// Filter taxonomic rank by sequences identity percentage
///
/// The filtration process should be based in different identity cutoff points.
/// This, be careful on set the taxon parameter.
pub(crate) fn filter_rank_by_identity(
    taxon: Taxon,
    perc_identity: f64,
    current_rank: ValidTaxonomicRanksEnum,
) -> Result<ValidTaxonomicRanksEnum, MappedErrors> {
    let rank = match taxon {
        Fungi => filter_fungi_identities(perc_identity),
        Bacteria => filter_bacteria_identities(perc_identity),
        Eukaryotes => filter_eukaryote_identities(perc_identity),
    };

    match rank {
        Err(err) => return Err(err),
        Ok(res) => {
            let ranks = ValidTaxonomicRanksEnum::ordered_iter(Some(true));

            let current_rank_index =
                ranks.to_owned().position(|rank| rank == &current_rank);

            let selected_rank_index =
                ranks.to_owned().position(|rank| rank == &res);

            if current_rank_index < selected_rank_index {
                return Ok(current_rank);
            }

            Ok(res)
        }
    }
}

/// Filter fungi ranks by identity percentage
///
/// TODO: Review the identity percentages and check a reference.
fn filter_fungi_identities(
    perc_identity: f64,
) -> Result<ValidTaxonomicRanksEnum, MappedErrors> {
    match perc_identity {
        i if i >= 97.0 => return Ok(Species),
        i if i >= 95.0 => return Ok(Genus),
        i if i >= 90.0 => return Ok(Family),
        i if i >= 85.0 => return Ok(Order),
        i if i >= 80.0 => return Ok(Class),
        i if i >= 75.0 => return Ok(Phylum),
        i if i >= 60.0 => return Ok(Domain),
        _ => return Ok(Undefined),
    };
}

/// Filter bacteria ranks by identity percentage
///
/// TODO: Review the identity percentages and check a reference.
fn filter_bacteria_identities(
    perc_identity: f64,
) -> Result<ValidTaxonomicRanksEnum, MappedErrors> {
    match perc_identity {
        i if i >= 99.0 => return Ok(Species),
        i if i >= 97.0 => return Ok(Genus),
        i if i >= 92.0 => return Ok(Family),
        i if i >= 85.0 => return Ok(Order),
        i if i >= 80.0 => return Ok(Class),
        i if i >= 75.0 => return Ok(Phylum),
        i if i >= 60.0 => return Ok(Domain),
        _ => return Ok(Undefined),
    };
}

/// Filter general eukaryotes ranks by identity percentage
///
/// TODO: Review the identity percentages and check a reference.
fn filter_eukaryote_identities(
    perc_identity: f64,
) -> Result<ValidTaxonomicRanksEnum, MappedErrors> {
    match perc_identity {
        i if i >= 97.0 => return Ok(Species),
        i if i >= 95.0 => return Ok(Genus),
        i if i >= 90.0 => return Ok(Family),
        i if i >= 85.0 => return Ok(Order),
        i if i >= 80.0 => return Ok(Class),
        i if i >= 75.0 => return Ok(Phylum),
        i if i >= 60.0 => return Ok(Domain),
        _ => return Ok(Undefined),
    };
}
