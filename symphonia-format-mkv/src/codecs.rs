// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::codecs;
use symphonia_core::codecs::CodecType;

use crate::segment::TrackElement;

pub(crate) fn codec_id_to_type(track: &TrackElement) -> Option<CodecType> {
    let bit_depth = track.audio.as_ref().and_then(|a| a.bit_depth);

    match track.codec_id.as_str() {
        "A_MPEG/L1" => Some(codecs::CODEC_TYPE_MP1),
        "A_MPEG/L2" => Some(codecs::CODEC_TYPE_MP2),
        "A_MPEG/L3" => Some(codecs::CODEC_TYPE_MP3),
        "A_FLAC" => Some(codecs::CODEC_TYPE_FLAC),
        "A_OPUS" => Some(codecs::CODEC_TYPE_OPUS),
        "A_VORBIS" => Some(codecs::CODEC_TYPE_VORBIS),
        "A_AAC/MPEG2/MAIN" | "A_AAC/MPEG2/LC" | "A_AAC/MPEG2/LC/SBR" | "A_AAC/MPEG2/SSR"
        | "A_AAC/MPEG4/MAIN" | "A_AAC/MPEG4/LC" | "A_AAC/MPEG4/LC/SBR" | "A_AAC/MPEG4/SSR"
        | "A_AAC/MPEG4/LTP" | "A_AAC" => Some(codecs::CODEC_TYPE_AAC),
        "A_PCM/INT/BIG" => match bit_depth? {
            16 => Some(codecs::CODEC_TYPE_PCM_S16BE),
            24 => Some(codecs::CODEC_TYPE_PCM_S24BE),
            32 => Some(codecs::CODEC_TYPE_PCM_S32BE),
            _ => None,
        },
        "A_PCM/INT/LIT" => match bit_depth? {
            16 => Some(codecs::CODEC_TYPE_PCM_S16LE),
            24 => Some(codecs::CODEC_TYPE_PCM_S24LE),
            32 => Some(codecs::CODEC_TYPE_PCM_S32LE),
            _ => None,
        },
        "A_PCM/FLOAT/IEEE" => match bit_depth? {
            32 => Some(codecs::CODEC_TYPE_PCM_F32LE),
            64 => Some(codecs::CODEC_TYPE_PCM_F64LE),
            _ => None,
        },

        "V_MS/VFW/FOURCC" => Some(codecs::CODEC_TYPE_VCM),
        "V_UNCOMPRESSED" => Some(codecs::CODEC_TYPE_VIDEO_UNCOMPRESSED),
        "V_MPEG4/ISO/SP" => Some(codecs::CODEC_TYPE_MPEG4_ISO_SP),
        "V_MPEG4/ISO/ASP" => Some(codecs::CODEC_TYPE_MPEG4_ISO_ASP),
        "V_MPEG4/ISO/AP" => Some(codecs::CODEC_TYPE_MPEG4_ISO_AP),
        "V_MPEG4/MS/V3" => Some(codecs::CODEC_TYPE_MPEG4_MS_V3),
        "V_MPEG1" => Some(codecs::CODEC_TYPE_MPEG1),
        "V_MPEG2" => Some(codecs::CODEC_TYPE_MPEG2),
        "V_MPEG4/ISO/AVC" => Some(codecs::CODEC_TYPE_MPEG4_ISO_AVC),
        "V_MPEGH/ISO/HEVC" => Some(codecs::CODEC_TYPE_MPEGH_ISO_HEVC),
        "V_AVS2" => Some(codecs::CODEC_TYPE_AVS2),
        "V_AVS3" => Some(codecs::CODEC_TYPE_AVS3),
        "V_REAL/RV10" => Some(codecs::CODEC_TYPE_RV10),
        "V_REAL/RV20" => Some(codecs::CODEC_TYPE_RV20),
        "V_REAL/RV30" => Some(codecs::CODEC_TYPE_RV30),
        "V_REAL/RV40" => Some(codecs::CODEC_TYPE_RV40),
        "V_QUICKTIME" => Some(codecs::CODEC_TYPE_QUICKTIME),
        "V_THEORA" => Some(codecs::CODEC_TYPE_THEORA),
        "V_PRORES" => Some(codecs::CODEC_TYPE_PRORES),
        "V_VP8" => Some(codecs::CODEC_TYPE_VP8),
        "V_VP9" => Some(codecs::CODEC_TYPE_VP9),
        "V_AV1" => Some(codecs::CODEC_TYPE_AV1),
        "V_FFV1" => Some(codecs::CODEC_TYPE_FFV1),

        _ => {
            log::info!("unknown codec: {}", &track.codec_id);
            None
        }
    }
}
